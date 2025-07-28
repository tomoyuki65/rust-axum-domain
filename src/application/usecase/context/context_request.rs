// axum
use axum::{extract::Request, http::header::HeaderMap};

// 共通コンテキストの構造体
#[derive(Clone, Debug)]
pub struct ContextRequest {
    pub header: HeaderMap,
    pub method: String,
    pub uri: String,
}

// リクエスト用コンテキストの作成
pub fn new_context_request(req: &Request) -> ContextRequest {
    let mut hm = HeaderMap::new();
    for (key, value) in req.headers().iter() {
        hm.insert(key.clone(), value.clone());
    }

    ContextRequest {
        header: hm,
        method: req.method().to_string(),
        uri: req.uri().to_string(),
    }
}
