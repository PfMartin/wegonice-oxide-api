use axum::{extract::State, http::StatusCode, routing::get, Json, Router};

use crate::{
    db::{generic_handler::GenericHandler, mongo_db_handler::MongoDbHandler},
    model::user::{User, UserMongoDb},
};

use super::api_response::ApiResponse;

pub struct UsersRouter {
    pub router: Router,
}

impl UsersRouter {
    pub fn new(db_handler: MongoDbHandler) -> Self {
        let router = Router::new()
            .route("/users", get(get_users))
            .with_state(db_handler);

        Self { router }
    }
}

async fn get_users(
    State(db_handler): State<MongoDbHandler>,
) -> (StatusCode, Json<ApiResponse<Vec<User>>>) {
    let get_users_result = db_handler.get_multiple::<UserMongoDb, User>("users").await;

    match get_users_result {
        Ok(users) => (
            StatusCode::OK,
            Json(ApiResponse {
                data: Some(users),
                error: "".into(),
            }),
        ),
        Err(err) => {
            let err_msg = "Failed to get multiple users";

            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    data: None,
                    error: err_msg.into(),
                }),
            );
        }
    }
}
