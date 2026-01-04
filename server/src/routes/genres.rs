use axum::{
    routing::get,
    Router,
};

use crate::handlers::genres::list_genres;

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/genres", get(list_genres))
}
