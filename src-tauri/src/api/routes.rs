//! API 路由定义
//!
//! 定义所有 API 端点及其处理函数

use axum::{
    routing::{get, post},
    Router,
};

use super::ApiServerMode;
use crate::api::CookieState;
use crate::api::handlers;

/// 创建 API 路由
pub fn create_routes(mode: ApiServerMode, cookie_state: CookieState) -> Router<CookieState> {
    let api_router = Router::new()
        // 健康检查
        .route("/health", get(handlers::health::health_check))
        // 视频 API
        .route("/api/video/:pickcode", get(handlers::video::get_video_info))
        .route(
            "/api/video/:pickcode/subtitle",
            get(handlers::video::get_video_subtitle),
        )
        .route(
            "/api/video/:pickcode/history",
            get(handlers::video::get_video_history),
        )
        .route(
            "/api/video/:pickcode/history",
            post(handlers::video::save_video_history),
        )
        // 文件 API
        .route("/api/files", get(handlers::files::list_files))
        .route(
            "/api/files/:pickcode",
            get(handlers::files::get_file_info),
        )
        // 用户 API
        .route("/api/user/info", get(handlers::user::get_user_info))
        .route("/api/user/quota", get(handlers::user::get_user_quota))
        // 下载任务 API
        .route("/api/downloads", get(handlers::download::list_downloads))
        .route("/api/downloads", post(handlers::download::create_download))
        // 上传任务 API
        .route("/api/uploads", get(handlers::upload::list_uploads))
        // 星标文件 API
        .route("/api/starred", get(handlers::starred::list_starred))
        .with_state(cookie_state.clone());

    // 根据模式决定根路径处理
    let app = if mode == ApiServerMode::Standalone {
        // 独立模式：直接使用 /api 前缀
        Router::new()
            .nest("/api", api_router)
            // 独立模式提供简单的前端页面
            .route("/", get(handlers::standalone::root_page))
            .route("/index.html", get(handlers::standalone::root_page))
            .with_state(cookie_state)
    } else {
        // 嵌入模式：所有路径都在 /api 下
        api_router
    };

    app
}
