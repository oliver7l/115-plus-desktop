//! Tauri 命令模块 — API 相关的命令
//!
//! 将命令放在独立模块中避免 lib.rs 中的 E0255 宏冲突

use crate::api;

/// 设置 115 API Cookie
///
/// 前端登录后调用此方法同步 Cookie 到 Rust 后端
#[tauri::command]
pub async fn set_cookies(
    cookies: String,
) -> Result<(), String> {
    api::get_cookie_state().set_cookies(cookies);
    log::info!("[API] Cookie 已更新");
    Ok(())
}
