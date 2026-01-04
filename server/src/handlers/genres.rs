use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct Genre {
    pub id: String,
    pub title: String,
}

pub async fn list_genres(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let res = sqlx::query_as!(
        Genre,
        r#"
        SELECT id, title
        FROM genres
        ORDER BY title
        "#
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(genres) => (StatusCode::OK, Json(genres)).into_response(),
        Err(err) => {
            eprintln!("Failed to list genres: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
