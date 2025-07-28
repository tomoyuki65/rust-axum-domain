// axum
use axum::{Router, middleware, routing::get};

// Arc（ヒープ上に確保されたある値の所有権を、複数のスレッド間で安全に共有するためのスマートポインタ）
use std::sync::Arc;

// tower_http
use tower_http::trace::TraceLayer;

// レジストリ
use crate::registry::registry_settings::AppState;

// ハンドラー
use crate::presentation::handler::user::user_handler;

// ミドルウェア
use crate::presentation::middleware::common_middleware;

pub fn router(state: Arc<AppState>) -> Router {
    // グループ設定「v1」
    let v1 = Router::new().route("/users", get(user_handler::find_all));

    // ルーター設定
    Router::new()
        .nest("/api/v1", v1)
        // 共通ミドルウェアの設定（下から順番に読み込み）
        .layer(middleware::from_fn(common_middleware::request_middleware))
        .layer(TraceLayer::new_for_http().on_response(
            |res: &axum::response::Response,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                // レスポンスヘッダーからX-Request-Idを取得
                let request_id = match res.headers().get("X-Request-Id") {
                    Some(value) => value.to_str().unwrap_or("-").to_string(),
                    None => "-".to_string(),
                };

                // ログ出力
                log::info!(
                    "[request_id={} status=({}) latency={}μs] finish request !!",
                    request_id,
                    res.status(),
                    latency.as_micros()
                )
            },
        ))
        .with_state(state)
}
