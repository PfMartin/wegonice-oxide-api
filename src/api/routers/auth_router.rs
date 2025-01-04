use axum::{
    extract::{self, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use tracing::info;

use crate::{
    api::services::hash_service::hash_password,
    db::{mongo_db_handler::MongoDbHandler, user_handler::UserHandler},
    model::user::{AuthPayload, UserCreate},
};

use super::super::api_response::ApiResponse;

pub struct AuthRouter {
    pub router: Router,
}

impl AuthRouter {
    pub fn new(db_handler: MongoDbHandler) -> Self {
        let base_path = "/auth";

        let router = Router::new()
            .route(&format!("{base_path}/register"), post(handle_register))
            .with_state(db_handler);

        Self { router }
    }
}

async fn handle_register(
    State(db_handler): State<MongoDbHandler>,
    Json(payload): extract::Json<AuthPayload>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let hashed_password = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(err) => {
            let err_msg = "Failed to process provided user data";
            info!("{err_msg}: {err}");

            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    data: None,
                    error: err_msg.into(),
                }),
            );
        }
    };

    let user_create = UserCreate {
        email: payload.email,
        password_hash: hashed_password,
    };

    match db_handler.create_user(user_create).await {
        Ok(inserted_id) => (
            StatusCode::CREATED,
            Json(ApiResponse {
                data: Some(inserted_id),
                error: "".into(),
            }),
        ),
        Err(err) => {
            let err_msg = "Failed to create new user";
            info!("{err_msg}: {err}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    data: None,
                    error: err_msg.into(),
                }),
            )
        }
    }
}
