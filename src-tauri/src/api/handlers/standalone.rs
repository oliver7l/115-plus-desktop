//! 独立模式前端页面

use axum::{http::StatusCode, response::Html};

/// GET /
///
/// 独立模式下的简单前端页面
pub async fn root_page() -> (StatusCode, Html<String>) {
    let html = r#"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>115+ API Service</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
            min-height: 100vh;
            color: #fff;
            padding: 40px;
        }
        .container { max-width: 900px; margin: 0 auto; }
        h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
            background: linear-gradient(90deg, #00d9ff, #00ff88);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        .subtitle { color: #888; margin-bottom: 40px; }
        .card {
            background: rgba(255,255,255,0.05);
            border-radius: 16px;
            padding: 30px;
            margin-bottom: 20px;
            border: 1px solid rgba(255,255,255,0.1);
        }
        .card h2 { font-size: 1.3em; margin-bottom: 15px; color: #00d9ff; }
        .endpoint {
            display: flex;
            align-items: center;
            padding: 12px;
            background: rgba(0,0,0,0.3);
            border-radius: 8px;
            margin-bottom: 8px;
            font-family: monospace;
        }
        .method {
            padding: 4px 10px;
            border-radius: 4px;
            font-weight: bold;
            margin-right: 12px;
            font-size: 0.85em;
        }
        .method.get { background: #00ff88; color: #000; }
        .method.post { background: #ff6b6b; color: #000; }
        .path { color: #ddd; }
        .desc { color: #888; font-size: 0.9em; margin-left: auto; }
        .footer {
            text-align: center;
            color: #555;
            margin-top: 40px;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>115+ API Service</h1>
        <p class="subtitle">115网盘 API 服务 - 基于 115-plus-desktop</p>

        <div class="card">
            <h2>健康检查</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/health</span>
                <span class="desc">服务健康状态</span>
            </div>
        </div>

        <div class="card">
            <h2>视频 API</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/video/:pickcode</span>
                <span class="desc">获取视频播放信息</span>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/video/:pickcode/subtitle</span>
                <span class="desc">获取字幕列表</span>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/video/:pickcode/history</span>
                <span class="desc">获取播放历史</span>
            </div>
            <div class="endpoint">
                <span class="method post">POST</span>
                <span class="path">/api/video/:pickcode/history</span>
                <span class="desc">保存播放进度</span>
            </div>
        </div>

        <div class="card">
            <h2>文件 API</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/files?cid=0</span>
                <span class="desc">列出文件</span>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/files/:pickcode</span>
                <span class="desc">获取文件详情</span>
            </div>
        </div>

        <div class="card">
            <h2>用户 API</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/user/info</span>
                <span class="desc">用户信息</span>
            </div>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/user/quota</span>
                <span class="desc">存储配额</span>
            </div>
        </div>

        <div class="card">
            <h2>星标文件</h2>
            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/api/starred</span>
                <span class="desc">星标文件列表</span>
            </div>
        </div>

        <div class="footer">
            Powered by 115-plus-desktop
        </div>
    </div>
</body>
</html>
"#.to_string();

    (StatusCode::OK, Html(html))
}
