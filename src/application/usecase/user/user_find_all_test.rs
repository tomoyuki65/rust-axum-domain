#[cfg(test)]
mod tests {
    use axum::{
        body::to_bytes,
        http::{HeaderMap, StatusCode},
    };
    use chrono::{FixedOffset, Utc};
    use std::{str, sync::Arc};

    // 共通コンテキスト
    use crate::application::usecase::context::context_request::ContextRequest;

    // ドメイン
    use crate::domain::{error::error_common::ErrorCommon, user::user_model::User};

    // ロガーのモック
    use crate::application::usecase::logger::logger_trait::MockLoggerTrait;

    // リポジトリのモック
    use crate::domain::user::user_repository::MockUserRepositoryTrait;

    // ユースケース
    use crate::application::usecase::user::user_find_all::UserFindAllRepository;
    use crate::application::usecase::user::user_find_all::UserFindAllUsecase;
    use crate::application::usecase::user::user_find_all::UserFindAllUsecaseTrait;

    #[tokio::test]
    async fn test_exec_success() {
        // ロガーのモック化
        let mut mock_logger = MockLoggerTrait::new();
        mock_logger.expect_info().returning(|_, _| ());

        // リポジトリのモック化
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap());
        let users = vec![
            User {
                id: 1,
                uid: "xxxx-xxxx-xxxx-0001".to_string(),
                last_name: "田中".to_string(),
                first_name: "太郎".to_string(),
                email: "t.tanaka@example.com".to_string(),
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
            User {
                id: 2,
                uid: "xxxx-xxxx-xxxx-0002".to_string(),
                last_name: "佐藤".to_string(),
                first_name: "二郎".to_string(),
                email: "z.satou@example.com".to_string(),
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
        ];
        mock_user_repo
            .expect_find_all()
            .returning(move |_| Ok(users.clone()));

        // ユースケースのインスタンス化
        let user_find_all_usecase = UserFindAllUsecase {
            repo: UserFindAllRepository {
                user_repository: Arc::new(mock_user_repo),
            },
            logger: Arc::new(mock_logger),
        };

        // 共通コンテキスト設定
        let mut h = HeaderMap::new();
        h.insert("X-Request-Id", "xxx-yyy-zzz-001".parse().unwrap());

        let ctx = ContextRequest {
            header: h,
            method: "GET".to_string(),
            uri: "/api/v1/users".to_string(),
        };

        // テスト実行
        let res = user_find_all_usecase.exec(ctx).await;

        // 検証
        assert_eq!(res.status(), StatusCode::OK);

        // レスポンスボディの検証
        let body = res.into_body();
        let bytes = to_bytes(body, usize::MAX).await.unwrap();
        let body_str = std::str::from_utf8(&bytes).unwrap();
        let res_data: Vec<User> = serde_json::from_str(body_str).unwrap();
        assert_eq!(res_data.len(), 2);

        assert_eq!(res_data[0].id, 1);
        assert_eq!(res_data[0].uid, "xxxx-xxxx-xxxx-0001".to_string());
        assert_eq!(res_data[0].last_name, "田中".to_string());
        assert_eq!(res_data[0].first_name, "太郎".to_string());
        assert_eq!(res_data[0].email, "t.tanaka@example.com".to_string());
        assert!(
            res_data[0].created_at
                <= Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())
        );
        assert_eq!(res_data[0].updated_at, res_data[0].created_at);
        assert!(res_data[0].deleted_at.is_none());

        assert_eq!(res_data[1].id, 2);
        assert_eq!(res_data[1].uid, "xxxx-xxxx-xxxx-0002".to_string());
        assert_eq!(res_data[1].last_name, "佐藤".to_string());
        assert_eq!(res_data[1].first_name, "二郎".to_string());
        assert_eq!(res_data[1].email, "z.satou@example.com".to_string());
        assert!(
            res_data[1].created_at
                <= Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())
        );
        assert_eq!(res_data[1].updated_at, res_data[1].created_at);
        assert!(res_data[1].deleted_at.is_none());
    }

    #[tokio::test]
    async fn test_exec_error() {
        // ロガーのモック化
        let mut mock_logger = MockLoggerTrait::new();
        mock_logger.expect_error().returning(|_, _| ());

        // リポジトリのモック化
        let mut mock_user_repo = MockUserRepositoryTrait::new();
        let err = ErrorCommon::InternalServerError;
        mock_user_repo
            .expect_find_all()
            .returning(move |_| Err(err.clone()));

        // ユースケースのインスタンス化
        let user_find_all_usecase = UserFindAllUsecase {
            repo: UserFindAllRepository {
                user_repository: Arc::new(mock_user_repo),
            },
            logger: Arc::new(mock_logger),
        };

        // 共通コンテキスト設定
        let mut h = HeaderMap::new();
        h.insert("X-Request-Id", "xxx-yyy-zzz-001".parse().unwrap());

        let ctx = ContextRequest {
            header: h,
            method: "GET".to_string(),
            uri: "/api/v1/users".to_string(),
        };

        // テスト実行
        let res = user_find_all_usecase.exec(ctx).await;

        // 検証
        assert_eq!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);

        // レスポンスボディの検証
        let body = res.into_body();
        let bytes = to_bytes(body, usize::MAX).await.unwrap();
        let body_str = str::from_utf8(&bytes).unwrap();
        assert!(body_str.contains("Internal Server Error"));
    }
}
