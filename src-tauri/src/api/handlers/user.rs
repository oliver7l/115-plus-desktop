//! 用户相关 API 处理函数

use axum::Json;
use serde::Serialize;

use crate::api::{client, ApiError, get_cookie_state};

/// 用户信息响应
#[derive(Serialize)]
pub struct UserInfoResponse {
    pub uid: String,
    pub user_name: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub avatar: Option<String>,
}

/// 用户配额响应
#[derive(Serialize)]
pub struct UserQuotaResponse {
    pub space_used: u64,
    pub space_total: u64,
}

/// GET /api/user/info
///
/// 获取用户信息
pub async fn get_user_info() -> Result<Json<UserInfoResponse>, ApiError> {
    let cookies_str = get_cookie_state().get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    let info = client::get_user_info(&cookies_str).await?;

    Ok(Json(UserInfoResponse {
        uid: info.uid,
        user_name: info.user_name,
        email: info.email,
        mobile: info.mobile,
        avatar: info.avatar,
    }))
}

/// GET /api/user/quota
///
/// 获取用户配额
pub async fn get_user_quota() -> Result<Json<UserQuotaResponse>, ApiError> {
    let cookies_str = get_cookie_state().get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    let quota = client::get_user_quota(&cookies_str).await?;

    let parse_size = |s: &str| -> u64 {
        s.parse::<u64>().unwrap_or(0)
    };

    Ok(Json(UserQuotaResponse {
        space_used: parse_size(&quota.space_used),
        space_total: parse_size(&quota.space_total),
    }))
}
