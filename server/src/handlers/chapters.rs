use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct Chapter {
    pub id: String,
    pub book_id: String,
    pub order_no: i32,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub position: i32,
}

#[derive(Deserialize)]
pub struct NewChapter {
    pub book_id: String,
    pub order_no: i32,
    pub title: String,
    pub content: String,
}

pub async fn list_chapters_by_book(
    Path(book_id): Path<String>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(
        Chapter,
        r#"
        SELECT
            id,
            book_id,
            order_no,
            title,
            content,
            created_at,
            position
        FROM chapters
        WHERE book_id = $1
        ORDER BY position
        "#,
        book_id
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(chapters) => (StatusCode::OK, Json(chapters)).into_response(),
        Err(err) => {
            eprintln!("Failed to list chapters: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_chapter(
    State(pool): State<PgPool>,
    Json(payload): Json<NewChapter>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();

    // 🔹 Bu kitap için sıradaki position
    let position: i32 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(MAX(position), 0) + 1
        FROM chapters
        WHERE book_id = $1
        "#
    )
    .bind(&payload.book_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let res = sqlx::query!(
        r#"
        INSERT INTO chapters (
            id,
            book_id,
            order_no,
            title,
            content,
            position
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        payload.book_id,
        payload.order_no,
        payload.title,
        payload.content,
        position
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(err) => {
            eprintln!("Failed to create chapter: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_chapter(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let res = sqlx::query!(
        r#"
        DELETE FROM chapters
        WHERE id = $1
        "#,
        id
    )
    .execute(&pool)
    .await;

    match res {
        Ok(result) if result.rows_affected() > 0 => {
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprintln!("Failed to delete chapter: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


pub async fn get_chapter(
    Path((book_id, id)): Path<(String, String)>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(
        Chapter,
        r#"
        SELECT
            id,
            book_id,
            order_no,
            title,
            content,
            created_at,
            position
        FROM chapters
        WHERE id = $1 AND book_id = $2
        "#,
        id,
        book_id
    )
    .fetch_optional(&pool)
    .await;

    match res {
        Ok(Some(chapter)) => (StatusCode::OK, Json(chapter)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprintln!("Failed to get chapter {} of book {}: {}", id, book_id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

