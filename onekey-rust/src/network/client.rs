//! HTTP 客户端模块

use anyhow::{Context, Result};
use reqwest::{Client, Response, header::HeaderMap};

/// HTTP 客户端封装
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .danger_accept_invalid_certs(true)
            .build()
            .context("创建 HTTP 客户端失败")?;

        Ok(Self { client })
    }

    /// GET 请求
    pub async fn get(&self, url: &str, headers: Option<&Vec<(String, String)>>) -> Result<Response> {
        let mut request = self.client.get(url);
        
        if let Some(headers) = headers {
            let mut header_map = HeaderMap::new();
            for (key, value) in headers {
                header_map.insert(
                    key.as_str(),
                    value.as_str().parse().unwrap_or_default(),
                );
            }
            request = request.headers(header_map);
        }

        request.send()
            .await
            .context(format!("GET 请求失败：{}", url))
    }

    /// 获取原始字节内容
    pub async fn get_bytes(&self, url: &str, headers: Option<&Vec<(String, String)>>) -> Result<Vec<u8>> {
        let response = self.get(url, headers).await?;
        let bytes = response.bytes().await.context("读取响应内容失败")?;
        Ok(bytes.to_vec())
    }
}
