use chrono::TimeZone;
use std::io::Write;

// ロガー用のトレイト
use crate::application::usecase::logger::logger_trait::LoggerTrait;

// 共通コンテキスト
use crate::application::usecase::context::context_request::ContextRequest;

// ロガーの構造体
#[derive(Clone)]
pub struct Logger {}

impl Logger {
    // ロガーの初期化処理
    pub fn init() {
        // 日本時間を取得
        let jst = chrono::offset::FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .from_utc_datetime(&chrono::Utc::now().naive_utc());

        // カスタムロガーの初期化
        env_logger::builder()
            .format(move |buf, record| {
                writeln!(
                    buf,
                    "{} {} {}",
                    jst.format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .init();
    }

    // インスタンス生成
    pub fn new() -> Self {
        Logger {}
    }

    // コンテキストからリクエスト情報取得
    fn get_req_info_from_ctx(ctx: &ContextRequest) -> String {
        // リクエストヘッダーから「X-Request-Id」を取得
        let x_request_id = ctx.header.get("X-Request-Id");
        let request_id = x_request_id.expect("-").to_str().unwrap();

        format!(
            "request_id={} method={} uri={}",
            request_id, ctx.method, ctx.uri
        )
    }
}

#[async_trait::async_trait]
impl LoggerTrait for Logger {
    fn info(&self, ctx: &ContextRequest, msg: &str) {
        let req_info = Logger::get_req_info_from_ctx(ctx);
        log::info!("[{}] {}", req_info, msg);
    }

    fn warn(&self, ctx: &ContextRequest, msg: &str) {
        let req_info = Logger::get_req_info_from_ctx(ctx);
        log::warn!("[{}] {}", req_info, msg);
    }

    fn error(&self, ctx: &ContextRequest, msg: &str) {
        let req_info = Logger::get_req_info_from_ctx(ctx);
        log::error!("[{}] {}", req_info, msg);
    }
}
