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
pub struct Book {
    pub id: String,
    pub genre_id: String,
    pub title: String,
    pub author: String,
    pub info: Option<String>,
    pub description: Option<String>,
    pub historical_context: Option<String>,
    pub author_bio: Option<String>,
    pub image_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct NewBook {
    pub genre_id: String,
    pub title: String,
    pub author: String,
    pub info: Option<String>,
    pub description: Option<String>,
    pub historical_context: Option<String>,
    pub author_bio: Option<String>,
    pub image_url: Option<String>,
}

pub async fn list_books(State(pool): State<PgPool>) -> impl IntoResponse {
    let res = sqlx::query_as!(
        Book,
        r#"
        SELECT
            id,
            genre_id,
            title,
            author,
            info,
            description,
            historical_context,
            author_bio,
            image_url,
            created_at
        FROM books
        ORDER BY title
        "#
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(books) => (StatusCode::OK, Json(books)).into_response(),
        Err(err) => {
            eprintln!("Failed to list books: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn create_book(
    State(pool): State<PgPool>,
    Json(payload): Json<NewBook>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();

    let res = sqlx::query!(
        r#"
        INSERT INTO books (id, genre_id, title, author, info, description, historical_context, author_bio, image_url)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        id,
        payload.genre_id,
        payload.title,
        payload.author,
        payload.info,
        payload.description,
        payload.historical_context,
        payload.author_bio,
        payload.image_url,
    )
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (StatusCode::CREATED, Json(id)).into_response(),
        Err(err) => {
            eprintln!("Failed to create book: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_book(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let res = sqlx::query!("DELETE FROM books WHERE id = $1", id)
        .execute(&pool)
        .await;

    match res {
        Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprintln!("Failed to delete book: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_book(
    Path(id): Path<String>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(
        Book,
        r#"
        SELECT
            id,
            genre_id,
            title,
            author,
            info,
            description,
            historical_context,
            author_bio,
            image_url,
            created_at
        FROM books
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await;

    match res {
        Ok(Some(book)) => (StatusCode::OK, Json(book)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprintln!("Failed to get book {}: {}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
