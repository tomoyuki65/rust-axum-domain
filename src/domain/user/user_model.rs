use chrono::{DateTime, FixedOffset, TimeZone};
use serde::{Deserialize, Serialize};

// ユーザーモデルの定義
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub uid: String,
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub deleted_at: Option<DateTime<FixedOffset>>,
}

impl User {
    // 新規作成
    #[allow(dead_code)]
    pub fn new(
        new_uid: String,
        new_last_name: String,
        new_first_name: String,
        new_email: String,
    ) -> Self {
        // jstの設定
        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();

        // 現在日時の設定
        let utc_now = chrono::Utc::now();
        let jst_now = jst_offset.from_utc_datetime(&utc_now.naive_utc());

        Self {
            id: 0,
            uid: new_uid,
            last_name: new_last_name,
            first_name: new_first_name,
            email: new_email,
            created_at: jst_now,
            updated_at: jst_now,
            deleted_at: None,
        }
    }

    // プロフィール更新
    #[allow(dead_code)]
    pub fn update_profile(
        &mut self,
        last_name: String,
        first_name: String,
        email: String,
    ) -> Result<(), String> {
        // パラメータチェック
        let mut err_msgs = Vec::new();
        if last_name.is_empty() {
            err_msgs.push("last_nameは必須です。");
        }
        if first_name.is_empty() {
            err_msgs.push("first_nameは必須です。");
        }
        if email.is_empty() {
            err_msgs.push("emailは必須です。");
        }
        if !err_msgs.is_empty() {
            let msg = err_msgs.join(", ");

            return Err(msg);
        }

        // 対象項目の更新
        self.last_name = last_name;
        self.first_name = first_name;
        self.email = email;

        // 現在日時の設定
        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
        let utc_now = chrono::Utc::now();
        let jst_now = jst_offset.from_utc_datetime(&utc_now.naive_utc());

        // 更新日時の更新
        self.updated_at = jst_now;

        Ok(())
    }

    // 論理削除設定
    #[allow(dead_code)]
    pub fn set_delete(&mut self) {
        // 現在日時の設定
        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
        let utc_now = chrono::Utc::now();
        let jst_now = jst_offset.from_utc_datetime(&utc_now.naive_utc());

        // 更新日時と削除日時の更新
        self.updated_at = jst_now;
        self.deleted_at = Some(jst_now);
    }
}
