<div align="center">

![Onekey](https://socialify.git.ci/ikunshare/Onekey/image?description=1&font=Inter&forks=1&issues=1&language=1&name=1&owner=1&pulls=1&stargazers=1&theme=Auto)

![GitHub Repo Size](https://img.shields.io/github/repo-size/ikunshare/Onekey?style=for-the-badge)
[![GitHub Release (with filter)](https://img.shields.io/github/v/release/ikunshare/Onekey?style=for-the-badge)](https://github.com/ikunshare/Onekey/releases/latest)
[![GitHub All Releases](https://img.shields.io/github/downloads/ikunshare/Onekey/total?style=for-the-badge&color=violet)](https://github.com/ikunshare/Onekey/releases)
[![GitHub License](https://img.shields.io/github/license/ikunshare/Onekey?style=for-the-badge)](https://github.com/ikunshare/Onekey/blob/main/LICENSE)

[![Powered by DartNode](https://dartnode.com/branding/DN-Open-Source-sm.png)](https://dartnode.com "Powered by DartNode - Free VPS for Open Source")

</div>

## Onekey - Rust 现代化重构版

> **本项目 fork 自** https://github.com/ikunshare/Onekey/  
> **为免费的旧版 onekey 进行现代化 Rust 改造**

Onekey Steam Depot Manifest Downloader - 使用 Rust 语言重新打造的现代化版本，提供 CLI 和 GUI 两种使用方式。

### 🚀 主要特性

- ✨ **现代化重构**：使用 Rust 语言完全重写，性能更优，内存更安全
- 🖥️ **双界面支持**：提供 CLI（命令行）和 GUI（图形界面）两种使用方式
- 🎨 **精美 GUI**：基于 Iced 框架 + Fluent Theme，提供现代化的 Windows 原生体验
- 🌏 **智能 CDN**：自动检测地区，中国大陆用户自动切换至加速 CDN
- 🔧 **工具支持**：支持 SteamTools 和 GreenLuma 两种解锁工具
- 📝 **配置管理**：自动读取 Steam 安装路径，支持自定义配置

### 📦 使用方法

#### GUI 版本（推荐）

1. 从 [Releases](https://github.com/ikunshare/Onekey/releases) 下载最新的 `onekey-gui.exe`
2. 确保已安装 SteamTools 或 GreenLuma
3. 运行程序，输入游戏 App ID
4. 选择解锁工具，点击"开始解锁"
5. 重启 Steam 以应用更改

#### CLI 版本

1. 从 [Releases](https://github.com/ikunshare/Onekey/releases) 下载最新的 `onekey.exe`
2. 编辑 `config.json` 配置文件，填入 GitHub Token
3. 运行程序，按照提示输入 App ID 和选择工具

### 🛠️ 开发构建

本程序使用 Rust 编程语言开发

**环境要求:**
- Rust 1.70+ (推荐使用 rustup 安装)
- Windows 10/11
- Git

#### 克隆项目

```bash
git clone https://github.com/ikunshare/Onekey
cd onekey-rust
```

#### 构建项目

```bash
# 构建所有目标
cargo build --release

# 仅构建 CLI 版本
cargo build --release --bin onekey

# 仅构建 GUI 版本
cargo build --release --bin onekey-gui
```

#### 运行测试

```bash
cargo test
```

### ⚙️ 配置说明

首次运行前需要创建 `config.json` 配置文件：

```json
{
  "Github_Personal_Token": "你的 GitHub Token",
  "Custom_Steam_Path": "",
  "Debug_Mode": false,
  "Logging_Files": true,
  "Help": "Github Personal Token 可在 GitHub 设置的 Developer settings 中生成"
}
```

### 📋 系统要求

- **操作系统**: Windows 10/11 (64-bit)
- **依赖**: 
  - Steam 客户端
  - SteamTools 或 GreenLuma（二选一）

### ⚠️ 免责声明

对本软件有意见的，欢迎拨打中华人民共和国公安部门报警电话：110 进行报警

#### 项目协议

本项目基于 GPL-2.0 许可证发行，以下协议是对于 GPL-2.0 原协议的补充，如有冲突，以以下协议为准。

- 本项目的数据来源原理是从 Steam 官方的 CDN 服务器中拉取游戏清单数据，经过对数据简单地筛选与合并后进行展示，因此本项目不对数据的准确性负责。
- 使用本项目的过程中可能会产生版权数据，对于这些版权数据，本项目不拥有它们的所有权，为了避免造成侵权，使用者务必在 24 小时内清除使用本项目的过程中所产生的版权数据。
- 本项目完全免费，且开源发布于 GitHub 面向全世界人用作对技术的学习交流。
- 禁止在违反当地法律法规的情况下使用本项目。
- 若你使用了本项目，将代表你接受以上协议。

**Steam 正版平台不易，请尊重版权，支持正版。**  
**本项目仅用于对技术可行性的探索及研究，不接受任何商业合作。**

### 📊 Star 趋势图

[![Stargazers over time](https://starchart.cc/ikunshare/Onekey.svg)](https://starchart.cc/ikunshare/Onekey)

### 🤝 贡献者

<a href="https://github.com/ikunshare/Onekey/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=ikunshare/Onekey" />
</a>

### 📞 社区和支持

加入我们的社区，参与讨论和支持:
- [GitHub Discussions](https://github.com/ikunshare/Onekey/discussions)
- [Telegram](https://t.me/ikunshare_qun)
- [QQ](https://qm.qq.com/q/NPRVbglteK)
