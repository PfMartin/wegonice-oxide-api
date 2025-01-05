use axum::{
    extract::{self, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use tracing::info;

use crate::{
    api::services::hash_service::verify_password_hash,
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
            .route(&format!("{base_path}/login"), post(handle_login))
            .with_state(db_handler);

        Self { router }
    }
}

async fn handle_register(
    State(db_handler): State<MongoDbHandler>,
    Json(payload): extract::Json<AuthPayload>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let user_create: UserCreate = match payload.try_into() {
        Ok(u) => u,
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

    match db_handler.create_user(user_create).await {
        Ok(inserted_id) => (
            // TODO: SEND EMAIL
            StatusCode::ACCEPTED,
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

async fn handle_login(
    State(db_handler): State<MongoDbHandler>,
    Json(payload): extract::Json<AuthPayload>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let user = match db_handler.get_user_by_email(&payload.email).await {
        Ok(u) => u,
        Err(err) => {
            let err_msg = format!(
                "Failed to find user with the provided email: {}",
                &payload.email
            );
            info!("{err_msg}: {err}");

            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    data: None,
                    error: err_msg,
                }),
            );
        }
    };

    match verify_password_hash(&payload.password, &user.password_hash) {
        Ok(_) => (
            // TODO: SET JWT TOKEN IN COOKIE
            StatusCode::ACCEPTED,
            Json(ApiResponse {
                data: None,
                error: "".into(),
            }),
        ),
        Err(err) => {
            let err_msg = format!(
                "Failed to find user with the provided email: {}",
                &payload.email
            );
            info!("{err_msg}: {err}");

            (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    data: None,
                    error: err_msg,
                }),
            )
        }
    }
}
