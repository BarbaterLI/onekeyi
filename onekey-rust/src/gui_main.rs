//! Onekey - Steam Depot Manifest Downloader
//! 
//! 本项目 fork 自 https://github.com/ikunshare/Onekey/
//! 为免费的旧版 onekey 进行现代化 Rust 改造
//! 
//! GUI 版本入口 (使用 Iced + Fluent Theme)

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Application, Command, Element, Length, Settings, Theme};

mod config;
mod constants;
mod logger;
mod models;
mod network;
mod tools;
mod utils;

use config::ConfigManager;
use logger::Logger;
use network::client::HttpClient;
use network::github::GitHubAPI;
use utils::region::RegionDetector;
use utils::steam::parse_key_file;
use tools::{SteamTools, GreenLuma, UnlockTool};

#[cfg(windows)]
use iced_fluent_theme::{FluentTheme, FluentStyle};

pub fn main() -> iced::Result {
    #[cfg(windows)]
    let theme = Theme::Custom(Box::new(FluentTheme::new(FluentStyle::Dark)));
    
    #[cfg(not(windows))]
    let theme = Theme::Light;
    
    OnekeyGui::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(650.0, 550.0),
            min_size: Some(iced::Size::new(450.0, 400.0)),
            ..Default::default()
        },
        theme,
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    AppIdInput(String),
    DownloadPressed,
    ToolSelected(ToolType),
    StatusUpdated(String),
    DownloadComplete(Result<bool>),
    RegionDetected(bool, String),
}

#[derive(Debug, Clone, PartialEq)]
enum ToolType {
    SteamTools,
    GreenLuma,
}

#[derive(Debug, Clone)]
enum Status {
    Idle,
    Loading,
    Success(String),
    Error(String),
}

struct OnekeyGui {
    app_id_input: String,
    selected_tool: Option<ToolType>,
    status: Status,
    config: Option<ConfigManager>,
    logger: Logger,
    is_cn: bool,
}

impl OnekeyGui {
    fn new() -> (Self, Command<Message>) {
        let config = ConfigManager::new().ok();
        let logger = Logger::new("onekey-gui", false, true);
        
        let mut gui = Self {
            app_id_input: String::new(),
            selected_tool: Some(ToolType::SteamTools),
            status: Status::Idle,
            config: config.clone(),
            logger,
            is_cn: true,
        };
        
        // 异步检测地区
        let command = Command::perform(async move {
            if let Ok(cfg) = config {
                let client = HttpClient::new().unwrap_or_else(|_| HttpClient::new().unwrap());
                let detector = RegionDetector::new(client, Logger::new("region", false, false));
                match detector.check_cn().await {
                    Ok((is_cn, country)) => (is_cn, country),
                    Err(_) => (true, "CN".to_string()),
                }
            } else {
                (true, "CN".to_string())
            }
        }, |(is_cn, country)| Message::RegionDetected(is_cn, country));
        
        (gui, command)
    }
    
    fn title(&self) -> String {
        format!("Onekey v{} - GUI Version", constants::VERSION)
    }
    
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AppIdInput(value) => {
                self.app_id_input = value;
                Command::none()
            }
            Message::ToolSelected(tool) => {
                self.selected_tool = Some(tool);
                Command::none()
            }
            Message::DownloadPressed => {
                if self.app_id_input.trim().is_empty() {
                    self.status = Status::Error("请输入有效的 App ID".to_string());
                    return Command::none();
                }
                
                self.status = Status::Loading;
                let app_id = self.app_id_input.clone();
                let tool_type = self.selected_tool.clone();
                let steam_path = self.config.as_ref()
                    .and_then(|c| c.steam_path.clone());
                
                Command::perform(Self::process_download(app_id, tool_type, steam_path), Message::DownloadComplete)
            }
            Message::StatusUpdated(msg) => {
                self.status = Status::Idle;
                Command::none()
            }
            Message::DownloadComplete(result) => {
                match result {
                    Ok(success) => {
                        if success {
                            self.status = Status::Success("解锁配置成功！请重启 Steam 以应用更改。".to_string());
                        } else {
                            self.status = Status::Error("解锁配置失败".to_string());
                        }
                    }
                    Err(e) => {
                        self.status = Status::Error(format!("错误：{}", e));
                    }
                }
                Command::none()
            }
            Message::RegionDetected(is_cn, country) => {
                self.is_cn = is_cn;
                self.logger.info(&format!("当前地区：{}", country));
                Command::none()
            }
        }
    }
    
    async fn process_download(
        app_id: String,
        tool_type: Option<ToolType>,
        steam_path: Option<std::path::PathBuf>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let config = ConfigManager::new()?;
        let logger = Logger::new("onekey-process", false, true);
        let http_client = HttpClient::new()?;
        
        let headers = config.get_github_headers();
        let github_api = GitHubAPI::new(http_client.clone(), headers, logger.clone());
        
        // 获取仓库信息
        let repo_info = github_api.get_latest_repo_info(&constants::REPO_LIST, &app_id).await?;
        
        match repo_info {
            Some(info) => {
                logger.info(&format!("找到最新仓库：{}", info.name));
                
                // 下载密钥文件
                let key_path = format!("keys/{}.txt", app_id);
                let key_content = github_api.fetch_file(&info.name, &info.sha, &key_path).await?;
                let depot_data = parse_key_file(&key_content);
                
                if depot_data.is_empty() {
                    return Err("未找到任何 Depot 信息".into());
                }
                
                logger.info(&format!("找到 {} 个 Depot", depot_data.len()));
                
                // 执行解锁
                let path = steam_path.ok_or("未找到 Steam 路径")?;
                
                match tool_type {
                    Some(ToolType::GreenLuma) => {
                        let greenluma = GreenLuma::new(path);
                        greenluma.setup(&depot_data, &app_id).await.map_err(|e| e.into())
                    }
                    _ => {
                        let steamtools = SteamTools::new(path);
                        steamtools.setup(&depot_data, &app_id).await.map_err(|e| e.into())
                    }
                }
            }
            None => Err("未找到该 App ID 的清单数据".into()),
        }
    }
    
    fn view(&self) -> Element<Message> {
        let title = text("Onekey Steam Depot Manifest Downloader")
            .size(28)
            .style(iced::theme::Text::Color([0.2, 0.6, 1.0].into()));
        
        let subtitle = text("本项目 fork 自 ikunshare/Onekey，为免费的旧版 onekey 进行现代化改造")
            .size(13)
            .style(iced::theme::Text::Color([0.7, 0.7, 0.7].into()));
        
        let app_id_label = text("游戏 App ID:").size(16);
        let app_id_input = text_input("输入 App ID...", &self.app_id_input)
            .on_input(Message::AppIdInput)
            .padding(12)
            .size(16);
        
        let input_row = row![app_id_label, app_id_input]
            .spacing(15)
            .align_items(iced::Alignment::Center);
        
        let steamtools_btn = button(text("SteamTools").size(14))
            .on_press(Message::ToolSelected(ToolType::SteamTools))
            .padding([12, 24])
            .style(if matches!(self.selected_tool, Some(ToolType::SteamTools)) {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            });
        
        let greenluma_btn = button(text("GreenLuma").size(14))
            .on_press(Message::ToolSelected(ToolType::GreenLuma))
            .padding([12, 24])
            .style(if matches!(self.selected_tool, Some(ToolType::GreenLuma)) {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            });
        
        let tool_row = row![steamtools_btn, greenluma_btn]
            .spacing(15)
            .padding([10, 0]);
        
        let download_btn = button(
            text(if matches!(self.status, Status::Loading) { 
                "处理中..." 
            } else { 
                "开始解锁" 
            }).size(16)
        )
            .on_press_maybe(if matches!(self.status, Status::Loading) { 
                None 
            } else { 
                Some(Message::DownloadPressed) 
            })
            .padding([14, 32])
            .style(iced::theme::Button::Success);
        
        let status_text = match &self.status {
            Status::Idle => text(""),
            Status::Loading => text("正在处理...").style(iced::theme::Text::Color([0.5, 0.5, 0.5].into())),
            Status::Success(msg) => text(msg).style(iced::theme::Text::Color([0.0, 0.8, 0.0].into())),
            Status::Error(msg) => text(msg).style(iced::theme::Text::Color([1.0, 0.2, 0.2].into())),
        };
        
        let content = column![
            title,
            subtitle,
            iced::widget::horizontal_rule(1),
            input_row,
            text("选择解锁工具:").size(16),
            tool_row,
            download_btn,
            status_text
        ]
        .spacing(18)
        .padding(30)
        .align_items(iced::Alignment::Center);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Application for OnekeyGui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    
    fn new() -> (Self, Command<Self::Message>) {
        Self::new()
    }
    
    fn title(&self) -> String {
        self.title()
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.update(message)
    }
    
    fn view(&self) -> Element<Self::Message> {
        self.view()
    }
}
