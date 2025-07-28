use std::sync::Arc;

// DB
use crate::infrastructure::database::database_dummy::new_db_dummy_connection;

// ロガー
use crate::infrastructure::logger::logger_log::Logger;

// リポジトリ
use crate::infrastructure::persistence::user::user_repository::UserRepository;

// ユースケース
use crate::application::usecase::user::user_find_all::UserFindAllRepository;
use crate::application::usecase::user::user_find_all::UserFindAllUsecase;

// Userユースケース
#[derive(Clone)]
pub struct UserUsecase {
    pub user_find_all: UserFindAllUsecase,
}

// アプリケーション全体で共有する状態（DIコンテナ）
#[derive(Clone)]
pub struct AppState {
    pub user_usecase: UserUsecase,
}

impl AppState {
    pub async fn new() -> Self {
        // DB設定
        let db = new_db_dummy_connection().await.unwrap();

        // ロガー設定
        let logger = Logger::new();

        // リポジトリのインスタンス化
        let user_repo = Arc::new(UserRepository::new(db, logger.clone()));

        // Userユースケースのインスタンス化とまとめ
        let user_find_all_repo = UserFindAllRepository {
            user_repository: user_repo,
        };
        let user_find_all_usecase = UserFindAllUsecase::new(user_find_all_repo, logger);
        let user_usecase = UserUsecase {
            user_find_all: user_find_all_usecase,
        };

        // 戻り値の設定
        Self { user_usecase }
    }
}
