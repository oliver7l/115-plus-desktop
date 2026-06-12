//! 视频相关 API 处理函数
//!
//! 直接调用 115 API 获取视频信息

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use log::info;

use crate::api::{client, ApiError, CookieState};

/// 视频信息响应
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfoResponse {
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
    pub subtitles: Vec<SubtitleInfo>,
}

/// 字幕信息
#[derive(serde::Serialize)]
pub struct SubtitleInfo {
    pub url: String,
    pub title: String,
    #[serde(rename = "sort")]
    pub sort: u32,
}

/// 视频历史记录响应
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoHistoryResponse {
    pub add_time: u64,
    pub file_id: String,
    pub file_name: String,
    pub hash: String,
    pub pick_code: String,
    pub time: u64,
}

/// 保存历史记录请求
#[derive(serde::Deserialize)]
pub struct SaveHistoryRequest {
    pub time: Option<u64>,
    pub watch_end: Option<u8>,
}

/// GET /api/video/:pickcode
///
/// 获取视频播放信息（HLS 地址等）
pub async fn get_video_info(
    Path(pickcode): Path<String>,
    State(cookies): State<CookieState>,
) -> Result<Json<VideoInfoResponse>, ApiError> {
    info!("[API] 获取视频信息: pickcode={}", pickcode);

    let cookies_str = cookies.get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    // 并行获取视频信息和字幕
    let (video_result, subtitle_result) = tokio::join!(
        client::get_video_info(&cookies_str, &pickcode),
        client::get_video_subtitle(&cookies_str, &pickcode)
    );

    let video = video_result?;

    let subtitles = subtitle_result.unwrap_or_default();

    Ok(Json(VideoInfoResponse {
        file_id: video.file_id,
        file_name: video.file_name,
        file_size: video.file_size,
        play_long: video.play_long,
        video_url: video.video_url,
        video_url_demo: video.video_url_demo,
        definition_list: video.definition_list,
        definition_index: video.definition_index,
        thumb_url: video.thumb_url,
        width: video.width,
        height: video.height,
        subtitles: subtitles
            .into_iter()
            .map(|s| SubtitleInfo {
                url: s.url,
                title: s.title,
                sort: s.sort,
            })
            .collect(),
    }))
}

/// GET /api/video/:pickcode/subtitle
///
/// 获取视频字幕列表
pub async fn get_video_subtitle(
    Path(pickcode): Path<String>,
    State(cookies): State<CookieState>,
) -> Result<Json<Vec<SubtitleInfo>>, ApiError> {
    info!("[API] 获取字幕列表: pickcode={}", pickcode);

    let cookies_str = cookies.get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    let subtitles = client::get_video_subtitle(&cookies_str, &pickcode).await?;

    Ok(Json(subtitles
        .into_iter()
        .map(|s| SubtitleInfo {
            url: s.url,
            title: s.title,
            sort: s.sort,
        })
        .collect()))
}

/// GET /api/video/:pickcode/history
///
/// 获取视频播放历史
pub async fn get_video_history(
    Path(pickcode): Path<String>,
    State(cookies): State<CookieState>,
) -> Result<Json<VideoHistoryResponse>, ApiError> {
    info!("[API] 获取视频历史: pickcode={}", pickcode);

    let cookies_str = cookies.get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    let history = client::get_video_history(&cookies_str, &pickcode).await?;

    Ok(Json(VideoHistoryResponse {
        add_time: history.add_time,
        file_id: history.file_id,
        file_name: history.file_name,
        hash: history.hash,
        pick_code: history.pick_code,
        time: history.time,
    }))
}

/// POST /api/video/:pickcode/history
///
/// 保存视频播放进度
pub async fn save_video_history(
    Path(pickcode): Path<String>,
    State(cookies): State<CookieState>,
    Json(payload): Json<SaveHistoryRequest>,
) -> Result<StatusCode, ApiError> {
    info!(
        "[API] 保存视频历史: pickcode={}, time={:?}",
        pickcode, payload.time
    );

    let cookies_str = cookies.get_cookies();
    if cookies_str.is_empty() {
        return Err(ApiError::Internal("未设置 Cookie，请先登录".to_string()));
    }

    client::save_video_history(&cookies_str, &pickcode, payload.time, payload.watch_end)
        .await?;

    Ok(StatusCode::OK)
}
