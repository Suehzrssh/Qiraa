#[cfg(feature = "sqlx-offline")]
const _: () = ();

mod handlers;
mod routes;

use axum::{
    routing::get,
    Router,
};

use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

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
  .allow_origin(Any)
  .allow_methods(Any)
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
    let port = std::env::var("PORT")
    .unwrap_or("3000".to_string())
    .parse::<u16>()
    .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(addr)
    .await
    .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
