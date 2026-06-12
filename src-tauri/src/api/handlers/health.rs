//! 健康检查接口

use axum::{http::StatusCode, Json};
use serde::Serialize;

/// 健康检查响应
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

/// GET /api/health
///
/// 返回服务健康状态
pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "healthy",
            version: env!("CARGO_PKG_VERSION"),
        }),
    )
}
