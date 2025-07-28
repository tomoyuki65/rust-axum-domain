use chrono::{FixedOffset, TimeZone};

// 共通コンテキスト
use crate::application::usecase::context::context_request::ContextRequest;

// ロガー
use crate::infrastructure::logger::logger_log::Logger;
// ロガーを使う場合はトレイトも読み込む
// use crate::application::usecase::logger::logger_trait::LoggerTrait;

// ドメイン
use crate::domain::{
    error::error_common::ErrorCommon, user::user_model::User,
    user::user_repository::UserRepositoryTrait,
};

// ユーザーリポジトリの構造体
pub struct UserRepository {
    pub _db: String, // TODO: 仮でString型にしているが、DBインスタンスに合わせた型に変更する
    pub _logger: Logger,
}

impl UserRepository {
    // 初期化用メソッド
    pub fn new(db: String, logger: Logger) -> Self {
        UserRepository {
            _db: db,
            _logger: logger,
        }
    }
}

#[async_trait::async_trait]
impl UserRepositoryTrait for UserRepository {
    // 全てのユーザー取得
    async fn find_all(&self, _ctx: &ContextRequest) -> Result<Vec<User>, ErrorCommon> {
        // *今回はDBを直接使わないため、固定値を返す*

        // jstの設定
        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();

        // 固定日付１
        let specific_datetime_1 = jst_offset.with_ymd_and_hms(2025, 7, 26, 7, 10, 10).unwrap();

        // 固定日付２
        let specific_datetime_2 = jst_offset.with_ymd_and_hms(2025, 7, 27, 8, 30, 0).unwrap();

        // ユーザーデータ
        let users: Vec<User> = vec![
            User {
                id: 1,
                uid: "xxxx-xxxx-xxxx-0001".to_string(),
                last_name: "田中".to_string(),
                first_name: "太郎".to_string(),
                email: "t.tanaka@example.com".to_string(),
                created_at: specific_datetime_1,
                updated_at: specific_datetime_1,
                deleted_at: None,
            },
            User {
                id: 2,
                uid: "xxxx-xxxx-xxxx-0002".to_string(),
                last_name: "佐藤".to_string(),
                first_name: "二郎".to_string(),
                email: "z.satou@example.com".to_string(),
                created_at: specific_datetime_2,
                updated_at: specific_datetime_2,
                deleted_at: None,
            },
        ];

        Ok(users)
    }
}
