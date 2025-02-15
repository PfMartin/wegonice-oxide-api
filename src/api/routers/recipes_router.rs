use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};

use crate::{
    api::{api_response::ApiResponse, services::token_service::decode_jwt},
    db::mongo_db_handler::MongoDbHandler,
};

#[derive(Clone)]
struct RouterState {
    jwt_secret: String,
    db_handler: MongoDbHandler,
}

pub struct RecipesRouter {
    pub router: Router,
}

impl RecipesRouter {
    pub fn new(db_handler: MongoDbHandler, jwt_secret: &str) -> Self {
        let base_path = "/recipes";

        let router_state = RouterState {
            db_handler,
            jwt_secret: String::from(jwt_secret),
        };

        let router = Router::new()
            .route(base_path, get(handle_get_recipes))
            .with_state(router_state);

        Self { router }
    }
}

async fn handle_get_recipes(
    State(router_state): State<RouterState>,
    headers: HeaderMap,
) -> (StatusCode, Json<ApiResponse<String>>) {
    println!("{}", router_state.db_handler.db.name());

    let token = match headers.get("wegonice-token") {
        Some(t) => match t.to_str() {
            Ok(t) => t,
            Err(e) => {
                println!("FAILED TO GET TOKEN");
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse {
                        data: None,
                        error: format!("Failed to get token from headers: {e}"),
                    }),
                );
            }
        },
        None => {
            println!("MISSING TOKEN");
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    data: None,
                    error: "Missing access token".into(),
                }),
            );
        }
    };

    let claims = match decode_jwt(token, &router_state.jwt_secret) {
        Ok(c) => c,
        Err(e) => {
            println!("FAILED TO GET CLAIMS");
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    data: None,
                    error: format!("Failed to get claims from token: {e}"),
                }),
            );
        }
    };

    (
        StatusCode::OK,
        Json(ApiResponse {
            data: Some(claims.sub),
            error: "".into(),
        }),
    )
}
