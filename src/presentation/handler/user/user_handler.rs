// axum
use axum::{
    extract::{Extension, State},
    response::Response,
};

// Arc（ヒープ上に確保されたある値の所有権を、複数のスレッド間で安全に共有するためのスマートポインタ）
use std::sync::Arc;

// レジストリ
use crate::registry::registry_settings::AppState;

// 共通コンテキスト
use crate::application::usecase::context::context_request::ContextRequest;

// ユースケースのトレイト
use crate::application::usecase::user::user_find_all::UserFindAllUsecaseTrait;

// ハンドラー
// 全てのユーザー取得
pub async fn find_all(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<ContextRequest>,
) -> Response {
    // ユースケースを実行
    state.user_usecase.user_find_all.exec(ctx).await
}
