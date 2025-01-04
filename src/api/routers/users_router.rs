use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use tracing::info;

use crate::{
    db::{
        generic_handler::GenericHandler, mongo_db_handler::MongoDbHandler,
        user_handler::UserHandler,
    },
    model::user::{User, UserMongoDb, UserPatch},
};

use super::super::api_response::ApiResponse;

pub struct UsersRouter {
    pub router: Router,
}

impl UsersRouter {
    pub fn new(db_handler: MongoDbHandler) -> Self {
        let router = Router::new()
            .route("/users", get(handle_users))
            .route("/users/{id}", get(handle_user_by_id))
            .route("/users/activate/{id}", patch(handle_activate_user))
            .route("/users/deactivate/{id}", patch(handle_deactivate_user))
            .with_state(db_handler);

        Self { router }
    }
}

async fn handle_users(
    State(db_handler): State<MongoDbHandler>,
) -> (StatusCode, Json<ApiResponse<Vec<User>>>) {
    match db_handler.get_multiple::<UserMongoDb, User>("users").await {
        Ok(users) => (
            StatusCode::OK,
            Json(ApiResponse {
                data: Some(users),
                error: "".into(),
            }),
        ),
        Err(err) => {
            let err_msg = "Failed to get multiple users";
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

async fn handle_user_by_id(
    State(db_handler): State<MongoDbHandler>,
    Path(user_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    match db_handler
        .get_by_id::<UserMongoDb, User>(&user_id, "users")
        .await
    {
        Ok(user) => (
            StatusCode::OK,
            Json(ApiResponse {
                data: Some(user),
                error: "".into(),
            }),
        ),
        Err(err) => {
            let err_msg = format!("Failed to get user with id '{user_id}'");
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

async fn handle_activate_user(
    State(db_handler): State<MongoDbHandler>,
    Path(user_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    match db_handler
        .patch_user_by_id(
            &user_id,
            UserPatch {
                email: None,
                password_hash: None,
                role: None,
                is_activated: Some(true),
            },
        )
        .await
    {
        Ok(_) => (
            StatusCode::NO_CONTENT,
            Json(ApiResponse {
                data: None,
                error: "".into(),
            }),
        ),
        Err(err) => {
            let err_msg = format!("Failed to activate user with id '{user_id}'");
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

async fn handle_deactivate_user(
    State(db_handler): State<MongoDbHandler>,
    Path(user_id): Path<String>,
) -> (StatusCode, Json<ApiResponse<String>>) {
    match db_handler
        .patch_user_by_id(
            &user_id,
            UserPatch {
                email: None,
                password_hash: None,
                role: None,
                is_activated: Some(false),
            },
        )
        .await
    {
        Ok(_) => (
            StatusCode::NO_CONTENT,
            Json(ApiResponse {
                data: None,
                error: "".into(),
            }),
        ),
        Err(err) => {
            let err_msg = format!("Failed to deactivate user with id '{user_id}'");
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
