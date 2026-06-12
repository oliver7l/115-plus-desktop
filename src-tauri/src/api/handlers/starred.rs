//! 星标文件相关 API 处理函数

use axum::Json;
use serde::Serialize;

use crate::api::{client, ApiError, get_cookie_state};

/// 星标文件项
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StarredItem {
    pub id: String,
    pub name: String,
    pub size: String,
    pub pick_code: String,
    pub ctime: String,
    pub ttime: String,
}

/// GET /api/starred
///
/// 获取星标文件列表
pub async fn list_starred() -> Result<Json<Vec<StarredItem>>, ApiError> {
    let cookies_str = get_cookie_state().get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    let result = client::list_starred(&cookies_str).await?;

    let items = result
        .list
        .into_iter()
        .map(|f| StarredItem {
            id: f.id,
            name: f.name,
            size: f.size,
            pick_code: f.pick_code,
            ctime: f.ctime,
            ttime: f.ttime,
        })
        .collect();

    Ok(Json(items))
}
