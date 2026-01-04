use axum::{routing::get, Router};
use crate::handlers::reader::read_full_book;

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/books/:book_id/read", get(read_full_book))
}
