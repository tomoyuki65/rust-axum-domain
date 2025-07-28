// 共通コンテキスト
use crate::application::usecase::context::context_request::ContextRequest;

// ロガーのトレイト（モック化もできるように定義）
#[mockall::automock]
#[async_trait::async_trait]
pub trait LoggerTrait: Send + Sync {
    fn info(&self, ctx: &ContextRequest, msg: &str);
    #[allow(dead_code)]
    fn warn(&self, ctx: &ContextRequest, msg: &str);
    #[allow(dead_code)]
    fn error(&self, ctx: &ContextRequest, msg: &str);
}
