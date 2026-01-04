use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::handlers::chapters::*;

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/chapters", post(create_chapter))
        .route("/chapters/:id", delete(delete_chapter))
        .route("/books/:book_id/chapters/:id", get(get_chapter))
        .route("/books/:book_id/chapters", get(list_chapters_by_book))
}
