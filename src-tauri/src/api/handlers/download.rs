//! 下载任务相关 API 处理函数
//!
//! 下载任务由 Rust 后端直接管理

use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

/// 下载任务项
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadTask {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub status: String,
    pub progress: f64,
    pub speed: String,
    pub size: String,
    pub created_at: String,
}

/// 创建下载任务请求
#[derive(Deserialize)]
pub struct CreateDownloadRequest {
    pub url: String,
    pub name: Option<String>,
    pub cid: Option<String>,
}

/// GET /api/downloads
///
/// 获取下载任务列表
pub async fn list_downloads(
) -> Result<Json<Vec<DownloadTask>>, ApiError> {
    // 下载任务由 Rust 下载模块管理
    // 由于 download 模块返回的是内部格式，这里先返回空列表
    // 后续可以添加实际的集成
    Ok(Json(vec![]))
}

/// POST /api/downloads
///
/// 创建下载任务
pub async fn create_download(
    Json(payload): Json<CreateDownloadRequest>,
) -> Result<StatusCode, ApiError> {
    // TODO: 集成 Rust 下载模块
    // 需要通过 Tauri commands 调用 download_enqueue_file
    Ok(StatusCode::CREATED)
}
