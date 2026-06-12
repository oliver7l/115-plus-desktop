//! 文件相关 API 处理函数

use axum::{
    extract::{Path, Query, State},
    Json,
};
use log::info;

use crate::api::{client, ApiError, CookieState};

/// 列出文件请求参数
#[derive(Deserialize)]
pub struct ListFilesQuery {
    pub cid: Option<String>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub sort: Option<String>,
    pub asc: Option<u8>,
}

use serde::Deserialize;

/// 文件项
#[derive(serde::Serialize)]
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
#[derive(serde::Serialize)]
pub struct FileListResponse {
    pub count: u64,
    pub offset: u64,
    pub list: Vec<FileItem>,
}

/// GET /api/files
///
/// 获取文件列表
pub async fn list_files(
    Query(params): Query<ListFilesQuery>,
    State(cookies): State<CookieState>,
) -> Result<Json<FileListResponse>, ApiError> {
    info!("[API] 列出文件: cid={:?}", params.cid);

    let cookies_str = cookies.get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    let cid = params.cid.unwrap_or_else(|| "0".to_string());
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(100);
    let sort = params.sort.as_deref().unwrap_or("file_name");
    let asc = params.asc.unwrap_or(1);

    let result = client::list_files(&cookies_str, &cid, Some(offset), Some(limit), Some(sort), Some(asc)).await?;

    let list = result
        .list
        .into_iter()
        .map(|f| FileItem {
            id: f.id,
            name: f.name,
            size: f.size,
            ctime: f.ctime,
            ttime: f.ttime,
            pick_code: f.pick_code,
            sha1: f.sha1,
            fid: f.fid,
        })
        .collect();

    Ok(Json(FileListResponse {
        count: result.count,
        offset: result.offset,
        list,
    }))
}

/// GET /api/files/:pickcode
///
/// 获取文件详情
pub async fn get_file_info(
    Path(pickcode): Path<String>,
    State(cookies): State<CookieState>,
) -> Result<Json<FileItem>, ApiError> {
    info!("[API] 获取文件详情: pickcode={}", pickcode);

    let cookies_str = cookies.get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    let file = client::get_file_info(&cookies_str, &pickcode).await?;

    Ok(Json(FileItem {
        id: file.id,
        name: file.name,
        size: file.size,
        ctime: file.ctime,
        ttime: file.ttime,
        pick_code: file.pick_code,
        sha1: file.sha1,
        fid: file.fid,
    }))
}
