// 共通コンテキスト
use crate::application::usecase::context::context_request::ContextRequest;

// ドメイン
use crate::domain::{error::error_common::ErrorCommon, user::user_model::User};

// Userリポジトリ用のトレイト（モック化もできるように定義）
// Send: オブジェクトが異なるスレッド間で安全に送信できることを保証
// Sync: オブジェクトが複数のスレッドから同時にアクセスできることを保証
#[mockall::automock]
#[async_trait::async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn find_all(&self, ctx: &ContextRequest) -> Result<Vec<User>, ErrorCommon>;
}
