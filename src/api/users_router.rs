use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use mongodb::Collection;

use crate::{db::mongo_db_handler::MongoDbHandler, model::user::UserMongoDb};

pub struct UsersRouter {
    pub router: Router,
}

impl UsersRouter {
    pub fn new(db_handler: MongoDbHandler) -> Self {
        let router = Router::new()
            .route("/users", get(handle_register))
            .with_state(db_handler);

        Self { router }
    }
}

async fn handle_register(State(db_handler): State<MongoDbHandler>) -> impl IntoResponse {
    Html("Hello Users")
}
