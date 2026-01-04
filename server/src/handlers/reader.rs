use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ReaderChapter {
    pub id: String,
    pub title: String,
    pub content: String,
    pub position: i32,
}

#[derive(Serialize)]
pub struct FullBook {
    pub book_id: String,
    pub title: String,
    pub chapters: Vec<ReaderChapter>,
}

pub async fn read_full_book(
    State(pool): State<PgPool>,
    Path(book_id): Path<String>,
) -> impl IntoResponse {
    let book = sqlx::query!(
        "SELECT id, title FROM books WHERE id = $1",
        book_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    let book = match book {
        Some(b) => b,
        None => return axum::http::StatusCode::NOT_FOUND.into_response(),
    };

    let chapters = sqlx::query_as!(
        ReaderChapter,
        r#"
        SELECT id, title, content, position
        FROM chapters
        WHERE book_id = $1
        ORDER BY position
        "#,
        book_id
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(FullBook {
        book_id: book.id,
        title: book.title,
        chapters,
    })
    .into_response()
}
