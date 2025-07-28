// ダミーのDB接続
pub async fn new_db_dummy_connection() -> Result<String, ()> {
    Ok("dummy".to_string())
}
