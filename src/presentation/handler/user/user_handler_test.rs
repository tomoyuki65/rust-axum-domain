#[cfg(test)]
mod tests {
    use crate::domain::user::user_model::User;

    #[tokio::test]
    async fn test_response_ok() {
        // リクエストを実行
        let url = "http://localhost:8080/api/v1/users";
        let client = reqwest::Client::new();
        let res = client.get(url).send().await.unwrap();

        // レスポンスステータスの検証
        assert_eq!(res.status(), 200);

        // レスポンスボディの検証
        let text_body = res.text().await.unwrap();
        let req_body: Vec<User> = serde_json::from_str(&text_body).unwrap();
        assert_eq!(req_body.len(), 2);

        assert_eq!(req_body[0].id, 1);
        assert_eq!(req_body[0].uid, "xxxx-xxxx-xxxx-0001");
        assert_eq!(req_body[0].last_name, "田中");
        assert_eq!(req_body[0].first_name, "太郎");
        assert_eq!(req_body[0].email, "t.tanaka@example.com");
        assert_eq!(req_body[0].created_at, req_body[0].updated_at);
        assert!(req_body[0].deleted_at.is_none());

        assert_eq!(req_body[1].id, 2);
        assert_eq!(req_body[1].uid, "xxxx-xxxx-xxxx-0002");
        assert_eq!(req_body[1].last_name, "佐藤");
        assert_eq!(req_body[1].first_name, "二郎");
        assert_eq!(req_body[1].email, "z.satou@example.com");
        assert_eq!(req_body[1].created_at, req_body[1].updated_at);
        assert!(req_body[1].deleted_at.is_none());
    }
}
