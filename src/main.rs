use axum::{
    Json, Router,
    extract::{Extension, Path},
    http::StatusCode,
    routing::get,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::{Level, info};

#[derive(Serialize, Deserialize)]
struct Url {
    id: i32,
    url_key: String,
    full_url: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUrl {
    full_url: String,
}

async fn get_urls(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Url>>, StatusCode> {
    let urls = sqlx::query_as!(Url, "SELECT id, url_key, full_url from urls")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(urls))
}

async fn get_url(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Url>, StatusCode> {
    let url = sqlx::query_as!(
        Url,
        "SELECT id, url_key, full_url from urls WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        info!("not found ");
        StatusCode::NOT_FOUND
    })?;

    Ok(Json(url))
}

async fn create_url(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_url): Json<CreateUrl>,
) -> Result<Json<Url>, StatusCode> {
    let url = sqlx::query_as!(
        Url,
        "INSERT INTO urls (url_key, full_url) VALUES ($1, $2) RETURNING id, full_url, url_key",
        &new_url.full_url,
        &new_url.full_url,
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        info!("tive erro {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(url))
}

async fn ping() -> &'static str {
    info!("request at /ping");
    "Pong"
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database");

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/urls", get(get_urls).post(create_url))
        .route("/urls/{id}", get(get_url))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is runinng on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
