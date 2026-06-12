//! 115 API 客户端
//!
//! 直接与 115 服务器通信，获取数据

use log::{error, info};
use reqwest::header::{COOKIE, REFERER};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// 115 API 客户端
pub struct ApiClient {
    http_client: reqwest::Client,
    cookies: String,
    base_url: String,
}

impl ApiClient {
    /// 创建新的 API 客户端
    pub fn new(cookies: String) -> Self {
        let http_client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            http_client,
            cookies,
            base_url: "https://proapi.115.com".to_string(),
        }
    }

    /// 设置 Cookie
    pub fn with_cookies(mut self, cookies: String) -> Self {
        self.cookies = cookies;
        self
    }

    /// 发起 GET 请求
    pub async fn get<T>(&self, path: &str, params: Option<Vec<(&str, &str)>>) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self.http_client.get(&url).header(COOKIE, &self.cookies);

        if let Some(params) = params {
            let query_string: Vec<String> = params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding(k), urlencoding(v)))
                .collect();
            let url = if query_string.is_empty() {
                url
            } else {
                format!("{}?{}", url, query_string.join("&"))
            };
            request = self.http_client.get(&url).header(COOKIE, &self.cookies);
        }

        let response = request.send().await.map_err(|e| ApiError::Network(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(ApiError::Http(status.as_u16(), response.text().await.ok()));
        }

        let json: Value = response.json().await.map_err(|e| ApiError::Parse(e.to_string()))?;

        if !json.get("state").and_then(|v| v.as_bool()).unwrap_or(false) {
            let code = json.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
            let message = json.get("message").or(json.get("error")).cloned();
            return Err(ApiError::Api(code, message));
        }

        serde_json::from_value(json).map_err(|e| ApiError::Parse(e.to_string()))
    }

    /// 发起 POST 请求
    pub async fn post<T>(&self, path: &str, data: Option<Value>) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .http_client
            .post(&url)
            .header(COOKIE, &self.cookies)
            .header(REFERER, "https://115.com/");

        if let Some(data) = data {
            let body = serde_urlencoded::to_string(&data)
                .map_err(|e| ApiError::Internal(e.to_string()))?;
            request = request
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body);
        }

        let response = request.send().await.map_err(|e| ApiError::Network(e.to_string()))?;

        let status = response.status();
        if !status.is_success() {
            return Err(ApiError::Http(status.as_u16(), response.text().await.ok()));
        }

        let json: Value = response.json().await.map_err(|e| ApiError::Parse(e.to_string()))?;

        serde_json::from_value(json).map_err(|e| ApiError::Parse(e.to_string()))
    }
}

// ============== 视频 API ==============

/// 视频播放信息响应
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    pub file_id: String,
    pub file_name: String,
    pub file_size: String,
    pub play_long: String,
    pub video_url: String,
    pub video_url_demo: String,
    pub definition_list: std::collections::HashMap<String, String>,
    pub definition_index: std::collections::HashMap<String, u32>,
    pub thumb_url: String,
    pub width: String,
    pub height: String,
}

/// 获取视频播放信息
pub async fn get_video_info(cookies: &str, pick_code: &str) -> Result<VideoInfo, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    client
        .get("/open/video/play", Some(vec![("pick_code", pick_code)]))
        .await
}

/// 字幕信息
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Subtitle {
    pub url: String,
    pub title: String,
    #[serde(rename = "sort")]
    pub sort: u32,
}

/// 字幕响应
#[derive(serde::Deserialize)]
pub struct SubtitleResponse {
    pub list: Vec<Subtitle>,
}

/// 获取字幕列表
pub async fn get_video_subtitle(cookies: &str, pick_code: &str) -> Result<Vec<Subtitle>, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    let response: SubtitleResponse = client
        .get("/open/video/subtitle", Some(vec![("pick_code", pick_code)]))
        .await?;
    Ok(response.list)
}

/// 播放历史响应
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoHistory {
    pub add_time: u64,
    pub file_id: String,
    pub file_name: String,
    pub hash: String,
    pub pick_code: String,
    pub time: u64,
}

/// 获取播放历史
pub async fn get_video_history(cookies: &str, pick_code: &str) -> Result<VideoHistory, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    client
        .get("/open/video/history", Some(vec![("pick_code", pick_code)]))
        .await
}

/// 保存播放历史
pub async fn save_video_history(
    cookies: &str,
    pick_code: &str,
    time: Option<u64>,
    watch_end: Option<u8>,
) -> Result<Value, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    let mut data = serde_json::json!({ "pick_code": pick_code });
    if let Some(t) = time {
        data["time"] = serde_json::json!(t);
    }
    if let Some(w) = watch_end {
        data["watch_end"] = serde_json::json!(w);
    }
    client.post("/open/video/history", Some(data)).await
}

// ============== 文件 API ==============

/// 文件项
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileItem {
    pub id: String,
    pub name: String,
    pub size: String,
    pub ctime: String,
    pub ttime: String,
    #[serde(rename = "pc")]
    pub pick_code: String,
    pub sha1: Option<String>,
    pub fid: Option<String>,
}

/// 文件列表响应
#[derive(Deserialize)]
pub struct FileListResponse {
    pub count: u64,
    pub offset: u64,
    pub list: Vec<FileItem>,
}

/// 列出文件
pub async fn list_files(
    cookies: &str,
    cid: &str,
    offset: Option<u64>,
    limit: Option<u64>,
    sort: Option<&str>,
    asc: Option<u8>,
) -> Result<FileListResponse, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    let offset_str = offset.unwrap_or(0).to_string();
    let limit_str = limit.unwrap_or(100).to_string();
    let asc_str = asc.unwrap_or(1).to_string();
    let params = vec![
        ("cid", cid),
        ("offset", &offset_str),
        ("limit", &limit_str),
        ("sort", sort.unwrap_or("file_name")),
        ("asc", &asc_str),
    ];
    client.get("/open/file/list", Some(params)).await
}

/// 获取文件详情
pub async fn get_file_info(cookies: &str, pick_code: &str) -> Result<FileItem, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    client
        .get(
            "/open/file/get",
            Some(vec![("pick_code", pick_code)]),
        )
        .await
}

// ============== 用户 API ==============

/// 用户信息
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub uid: String,
    pub user_name: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub avatar: Option<String>,
}

/// 获取用户信息
pub async fn get_user_info(cookies: &str) -> Result<UserInfo, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    client.get("/open/user/info", None).await
}

/// 用户配额
#[derive(Deserialize)]
pub struct UserQuota {
    pub space_used: String,
    pub space_total: String,
}

/// 获取用户配额
pub async fn get_user_quota(cookies: &str) -> Result<UserQuota, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    client.get("/open/user/space", None).await
}

// ============== 星标 API ==============

/// 星标文件响应
#[derive(Deserialize)]
pub struct StarredResponse {
    pub count: u64,
    pub list: Vec<FileItem>,
}

/// 获取星标文件
pub async fn list_starred(cookies: &str) -> Result<StarredResponse, ApiError> {
    let client = ApiClient::new(cookies.to_string());
    client.get("/open/starred/list", None).await
}

// ============== Error Types ==============

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("网络错误: {0}")]
    Network(String),

    #[error("HTTP 错误: {0} - {1:?}")]
    Http(u16, Option<String>),

    #[error("解析错误: {0}")]
    Parse(String),

    #[error("115 API 错误: code={0} - {1:?}")]
    Api(i64, Option<Value>),

    #[error("内部错误: {0}")]
    Internal(String),
}

impl serde::Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            ApiError::Network(_) => axum::http::StatusCode::BAD_GATEWAY,
            ApiError::Http(code, _) => {
                axum::http::StatusCode::from_u16(*code)
                    .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            ApiError::Parse(_) | ApiError::Internal(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            ApiError::Api(_, _) => axum::http::StatusCode::BAD_GATEWAY,
        };
        (status, self.to_string()).into_response()
    }
}

/// URL 编码辅助函数
fn urlencoding(s: &str) -> String {
    let mut result = String::new();
    for byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(*byte as char);
            }
            b' ' => result.push_str("+"),
            _ => result.push_str(&format!("%{:02X}", byte)),
        }
    }
    result
}
