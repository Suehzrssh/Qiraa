use axum::{
    routing::{get, post, delete},
    Router,
};

use crate::handlers::book::{list_books, create_book, delete_book, get_book};

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/books", get(list_books))
        .route("/books/:id", get(get_book))
        .route("/books/post", post(create_book))
        .route("/books/:id", delete(delete_book))
}
