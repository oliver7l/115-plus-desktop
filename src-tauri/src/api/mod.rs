//! HTTP API Server 模块
//!
//! 提供 REST API 接口，供外部调用访问 115 网盘功能
//!
//! # 架构
//!
//! - 使用 axum 构建 HTTP API 服务
//! - 直接调用 115 API，不依赖前端
//! - Cookie 通过前端同步到 Rust 状态

pub mod client;
pub mod handlers;
pub mod routes;

pub use client::ApiError;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Router, ServiceExt};
use log::{error, info};
use tokio::sync::broadcast;

/// API 服务配置
#[derive(Debug, Clone)]
pub struct ApiServerConfig {
    /// HTTP 服务地址
    pub host: String,
    /// HTTP 服务端口
    pub port: u16,
    /// 启用 CORS
    pub enable_cors: bool,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 11500,
            enable_cors: true,
        }
    }
}

/// API 服务模式
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ApiServerMode {
    /// 独立启动，不依赖前端窗口
    Standalone,
    /// 嵌入到主应用中，随窗口运行
    Embedded,
}

/// API 服务状态
#[derive(Debug)]
pub struct ApiServerState {
    /// 服务配置
    pub config: ApiServerConfig,
    /// 停止信号发送者
    pub shutdown_tx: broadcast::Sender<()>,
}

impl ApiServerState {
    pub fn new(config: ApiServerConfig) -> (Self, broadcast::Receiver<()>) {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        (
            Self {
                config,
                shutdown_tx,
            },
            shutdown_rx,
        )
    }
}

/// Cookie 管理状态
#[derive(Debug)]
pub struct CookieState {
    pub cookies: std::sync::RwLock<String>,
}

impl Clone for CookieState {
    fn clone(&self) -> Self {
        Self {
            cookies: std::sync::RwLock::new(self.get_cookies()),
        }
    }
}

impl Default for CookieState {
    fn default() -> Self {
        Self {
            cookies: std::sync::RwLock::new(String::new()),
        }
    }
}

impl CookieState {
    /// 设置 Cookie
    pub fn set_cookies(&self, cookies: String) {
        *self.cookies.write().unwrap() = cookies;
    }

    /// 获取 Cookie
    pub fn get_cookies(&self) -> String {
        self.cookies.read().unwrap().clone()
    }
}

/// 启动 API HTTP 服务
///
/// 返回服务句柄，可通过 `shutdown_tx` 停止服务
pub async fn start_server(
    config: ApiServerConfig,
    mode: ApiServerMode,
    cookie_state: CookieState,
) -> Result<ApiServerState, ApiServerError> {
    let (state, mut shutdown_rx) = ApiServerState::new(config.clone());

    let app = if config.enable_cors {
        use tower_http::cors::{Any, CorsLayer};
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        routes::create_routes(mode, cookie_state.clone()).layer(cors)
    } else {
        routes::create_routes(mode, cookie_state.clone())
    };

    let addr: SocketAddr = format!("{}:{}", config.host, config.port)
        .parse()
        .map_err(|e: std::net::AddrParseError| ApiServerError::InvalidAddress(e.to_string()))?;

    info!("[API] 启动 HTTP 服务于 http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| ApiServerError::BindFailed(e.to_string()))?;

    // 启动服务
    let server = axum::serve(listener, app);

    // 创建停止信号
    let shutdown_tx = state.shutdown_tx.clone();

    // 使用 spawn 来运行服务，这样不会阻塞
    tokio::spawn(async move {
        tokio::select! {
            result = server => {
                if let Err(e) = result {
                    error!("[API] HTTP 服务错误: {}", e);
                }
            }
            _ = shutdown_rx.recv() => {
                info!("[API] 收到停止信号，关闭 HTTP 服务");
            }
        }
    });

    // 注册服务地址到 Tauri state（用于前端显示）
    info!("[API] HTTP 服务已启动于 http://{}", addr);

    Ok(state)
}

/// 停止 API 服务
pub fn stop_server(state: &ApiServerState) {
    let _ = state.shutdown_tx.send(());
}

// ============== Error Types ==============

#[derive(Debug, thiserror::Error)]
pub enum ApiServerError {
    #[error("无法绑定地址: {0}")]
    BindFailed(String),

    #[error("无效的地址: {0}")]
    InvalidAddress(String),

    #[error("前端未连接: {0}")]
    FrontendNotConnected(String),

    #[error("请求超时: {0}")]
    Timeout(String),

    #[error("115 API 错误: {0}")]
    ApiError(String),

    #[error("内部错误: {0}")]
    Internal(String),
}

impl serde::Serialize for ApiServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
