use axum::{
    extract::{self, State},
    http::StatusCode,
    routing::post,
    Json, Router,
};
use tracing::info;

use crate::{
    api::services::{hash_service::verify_password_hash, token_service::generate_jwt},
    db::{mongo_db_handler::MongoDbHandler, user_handler::UserHandler},
    model::user::{AuthPayload, UserCreate},
};

use super::super::api_response::ApiResponse;

#[derive(Clone)]
struct RouterState {
    jwt_secret: String,
    db_handler: MongoDbHandler,
}

pub struct AuthRouter {
    pub router: Router,
}

impl AuthRouter {
    pub fn new(db_handler: MongoDbHandler, jwt_secret: &str) -> Self {
        let base_path = "/auth";

        let router_state = RouterState {
            db_handler,
            jwt_secret: String::from(jwt_secret),
        };

        let router = Router::new()
            .route(&format!("{base_path}/register"), post(handle_register))
            .route(&format!("{base_path}/login"), post(handle_login))
            .with_state(router_state);

        Self { router }
    }
}

async fn handle_register(
    State(router_state): State<RouterState>,
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

    match router_state.db_handler.create_user(user_create).await {
        Ok(inserted_id) => (
            // TODO: SEND EMAIL REGARDING VERIFICATION
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
    State(router_state): State<RouterState>,
    Json(payload): extract::Json<AuthPayload>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    let auth_info = match router_state
        .db_handler
        .get_user_auth_info(&payload.email)
        .await
    {
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

    if verify_password_hash(&payload.password, &auth_info.password_hash).is_err() {
        let err_msg = format!("Incorrect password: {}", &payload.email);
        info!("{err_msg}");

        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                data: None,
                error: err_msg,
            }),
        );
    }

    if !&auth_info.is_activated {
        let err_msg = format!("User is inactive: {}", &payload.email);
        info!("{err_msg}");

        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                data: None,
                error: err_msg,
            }),
        );
    }

    let token = match generate_jwt(&auth_info, 1, &router_state.jwt_secret) {
        Ok(t) => t,
        Err(err) => {
            let err_msg = "Failed to generate JWT token";
            info!("{err_msg}: {err}");

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    data: None,
                    error: err_msg.into(),
                }),
            );
        }
    };

    (
        // TODO: SET JWT TOKEN with role IN COOKIE
        StatusCode::ACCEPTED,
        Json(ApiResponse {
            data: Some(token),
            error: "".into(),
        }),
    )
}
