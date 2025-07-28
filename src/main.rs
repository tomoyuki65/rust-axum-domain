// axum
use axum::serve;

// Arc（ヒープ上に確保されたある値の所有権を、複数のスレッド間で安全に共有するためのスマートポインタ）
use std::sync::Arc;

// モジュールのインポート
mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;
mod registry;

// コンフィグ設定
use crate::config::config_settings::get_config;

// ルーター設定
use crate::presentation::router::router_settings::router;

// ロガー設定
use crate::infrastructure::logger::logger_log::Logger;

// レジストリ設定
use crate::registry::registry_settings::AppState;

#[tokio::main]
async fn main() {
    // 環境変数取得
    let config = get_config();

    // ロガーの初期化
    Logger::init();

    // サーバー起動のログ出力
    log::info!("Start rust_axum_domain (ENV:{}) !!", config.env);

    // サーバー起動
    let state = Arc::new(AppState::new().await);
    let app = router(state);
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}
