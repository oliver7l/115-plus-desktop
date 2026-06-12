//! 上传任务相关 API 处理函数
//!
//! 上传任务由 Rust 后端直接管理

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::api::ApiError;

/// 上传任务项
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadTask {
    pub id: String,
    pub name: String,
    pub status: String,
    pub progress: f64,
    pub size: String,
    pub created_at: String,
}

/// GET /api/uploads
///
/// 获取上传任务列表
pub async fn list_uploads() -> Result<Json<Vec<UploadTask>>, ApiError> {
    // 上传任务由 Rust 上传模块管理
    // 由于 upload 模块返回的是内部格式，这里先返回空列表
    // 后续可以添加实际的集成
    Ok(Json(vec![]))
}
