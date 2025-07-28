// axum
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Arc（ヒープ上に確保されたある値の所有権を、複数のスレッド間で安全に共有するためのスマートポインタ）
use std::sync::Arc;

// json変換用マクロ
use serde_json::json;

// 共通コンテキスト
use crate::application::usecase::context::context_request::ContextRequest;

// ドメイン
use crate::domain::error::error_common::ErrorCommon;
use crate::domain::user::user_repository::UserRepositoryTrait;

// ロガー
use crate::application::usecase::logger::logger_trait::LoggerTrait;

// ユースケース用のトレイト（モック化もできるように定義）
#[mockall::automock]
#[async_trait::async_trait]
pub trait UserFindAllUsecaseTrait {
    async fn exec(&self, ctx: ContextRequest) -> Response;
}

// 使用するリポジトリをまとめる構造体
#[derive(Clone)]
pub struct UserFindAllRepository {
    // Arc<T>型で動的にメモリ領域確保（スレッドセーフな共有所有権）
    // 'static: オブジェクトのライフタイムがプログラムが終了するまで破棄されない
    pub user_repository: Arc<dyn UserRepositoryTrait + 'static>,
}

// ユースケースの構造体
#[derive(Clone)]
pub struct UserFindAllUsecase {
    pub repo: UserFindAllRepository,
    pub logger: Arc<dyn LoggerTrait + 'static>,
}

impl UserFindAllUsecase {
    pub fn new(repo: UserFindAllRepository, logger: Arc<dyn LoggerTrait + 'static>) -> Self {
        UserFindAllUsecase { repo, logger }
    }
}

#[async_trait::async_trait]
impl UserFindAllUsecaseTrait for UserFindAllUsecase {
    async fn exec(&self, ctx: ContextRequest) -> Response {
        // レスポンスヘッダーに付与する値の設定
        let x_request_id = ctx.header.get("X-Request-Id");
        let request_id = x_request_id.expect("-").to_str().unwrap();
        let res_header = [("X-Request-Id", request_id)];

        // 全てのユーザー取得処理
        let users = match self.repo.user_repository.find_all(&ctx).await {
            Ok(users) => users,
            Err(err) => {
                // エラーログ出力
                let err_msg = format!("UserFindAllUsecaseでエラー: {}", err);
                self.logger.error(&ctx, &err_msg);

                // json形式のメッセージを設定
                let json_msg = Json(json!({ "message": err.to_string()}));

                // ステータスコードの設定
                let status_code = match err {
                    ErrorCommon::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorCommon::CustomError { status_code, .. } => status_code,
                };

                // レスポンス結果の設定
                let res = (status_code, res_header, json_msg).into_response();

                // 戻り値としてレスポンス結果を返す
                return res;
            }
        };

        // レスポンスボディの設定
        let res_body = Json(json!(users));

        // レスポンス結果を設定して戻り値として返す
        (StatusCode::OK, res_header, res_body).into_response()
    }
}
