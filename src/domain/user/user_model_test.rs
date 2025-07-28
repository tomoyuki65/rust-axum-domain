#[cfg(test)]
mod tests {
    use crate::domain::user::user_model::User;
    use chrono::{FixedOffset, Utc};

    #[test]
    fn test_new_user() {
        // ユーザーのパラメータ
        let uid = "xxx-xxx-xxx-0001".to_string();
        let last_name = "テスト".to_string();
        let first_name = "太郎".to_string();
        let email = "t.test@example.com".to_string();

        // テスト実行
        let user = User::new(
            uid.clone(),
            last_name.clone(),
            first_name.clone(),
            email.clone(),
        );

        // 検証
        assert_eq!(user.id, 0);
        assert_eq!(user.uid, uid);
        assert_eq!(user.last_name, last_name);
        assert_eq!(user.first_name, first_name);
        assert_eq!(user.email, email);
        assert!(
            user.created_at <= Utc::now().with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap())
        );
        assert_eq!(user.created_at, user.updated_at);
        assert!(user.deleted_at.is_none());
    }

    #[test]
    fn test_update_profile_success() {
        // ユーザーのパラメータ
        let uid = "xxx-xxx-xxx-0001".to_string();
        let last_name = "テスト".to_string();
        let first_name = "太郎".to_string();
        let email = "t.test@example.com".to_string();

        // ユーザー作成
        let mut user = User::new(
            uid.clone(),
            last_name.clone(),
            first_name.clone(),
            email.clone(),
        );

        // 更新するプロフィール
        let new_last_name = "テスト２".to_string();
        let new_first_name = "二郎".to_string();
        let new_email = "t.test2@example.com".to_string();

        // テスト実行
        let result = user.update_profile(
            new_last_name.clone(),
            new_first_name.clone(),
            new_email.clone(),
        );

        // 検証
        assert!(result.is_ok());
        assert_eq!(user.uid, uid);
        assert_eq!(user.last_name, new_last_name);
        assert_eq!(user.first_name, new_first_name);
        assert_eq!(user.email, new_email);
        assert!(user.updated_at > user.created_at);
        assert!(user.deleted_at.is_none());
    }

    #[test]
    fn test_update_profile_error() {
        // ユーザーのパラメータ
        let uid = "xxx-xxx-xxx-0001".to_string();
        let last_name = "テスト".to_string();
        let first_name = "太郎".to_string();
        let email = "t.test@example.com".to_string();

        // ユーザー作成
        let mut user = User::new(
            uid.clone(),
            last_name.clone(),
            first_name.clone(),
            email.clone(),
        );

        // 更新するプロフィール
        let new_last_name = "".to_string();
        let new_first_name = "".to_string();
        let new_email = "".to_string();

        // テスト実行
        let result = user.update_profile(
            new_last_name.clone(),
            new_first_name.clone(),
            new_email.clone(),
        );

        // 検証
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "last_nameは必須です。, first_nameは必須です。, emailは必須です。"
        );
        assert_eq!(user.uid, uid);
        assert_eq!(user.last_name, last_name);
        assert_eq!(user.first_name, first_name);
        assert_eq!(user.email, email);
        assert!(user.updated_at == user.created_at);
        assert!(user.deleted_at.is_none());
    }

    #[test]
    fn test_set_delete_success() {
        // ユーザーのパラメータ
        let uid = "xxx-xxx-xxx-0001".to_string();
        let last_name = "テスト".to_string();
        let first_name = "太郎".to_string();
        let email = "t.test@example.com".to_string();

        // ユーザー作成
        let mut user = User::new(
            uid.clone(),
            last_name.clone(),
            first_name.clone(),
            email.clone(),
        );

        // テスト実行
        user.set_delete();

        // 検証
        assert_eq!(user.uid, uid);
        assert_eq!(user.last_name, last_name);
        assert_eq!(user.first_name, first_name);
        assert_eq!(user.email, email);
        assert!(user.updated_at > user.created_at);
        assert!(user.deleted_at.is_some());
    }
}
