// axum
use axum::{extract::Request, middleware::Next, response::Response};

// UUID
use uuid::Uuid;

// 共通コンテキストのモジュール
use crate::application::usecase::context::context_request;

// ロガー設定
use crate::application::usecase::logger::logger_trait::LoggerTrait;
use crate::infrastructure::logger::logger_log::Logger;

// リクエスト用のミドルウェア
pub async fn request_middleware(mut req: Request, next: Next) -> Response {
    // リクエストヘッダー「X-Request-Id」にUUIDを設定
    let request_id = Uuid::new_v4().to_string();
    req.headers_mut()
        .insert("X-Request-Id", request_id.parse().unwrap());

    // リクエストに共通コンテキストのExtentionを追加
    let ctx = context_request::new_context_request(&req);
    req.extensions_mut().insert(ctx.clone());

    // リクエスト単位でログ出力
    let logger = Logger::new();
    logger.info(&ctx, "start request !!");

    next.run(req).await
}
