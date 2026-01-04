mod handlers;
mod routes;

use axum::{
    routing::get,
    Router,
};
use axum::http::Method;
use http::header::HeaderValue;
use sqlx::PgPool;
use std::env;
use tower_http::cors::{AllowMethods, Any, CorsLayer};

#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // Database
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // CORS
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::DELETE,
        ]))
        .allow_headers(Any);

    // App
    let app = Router::new()
        .route("/health", get(|| async { "Qira'a backend OK" }))
        .nest("/api", routes::book::router())
        .nest("/api", routes::chapters::router())
        .nest("/api", routes::reader::router())
        .nest("/api", routes::genres::router())
        .with_state(pool)
        .layer(cors);

    // Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
