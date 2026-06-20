use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

#[derive(Clone, Serialize, FromRow)]
struct Data {
    name: String,
    description: String,
    age: i32,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    const DATABASE_URL: &str = "postgresql://user:password@database:5432/axum_database_benchmarking";
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    let app = Router::new()
        .route("/greeting", get(greeting))
        .route("/request", get(request))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn greeting() -> String {
    "Hello, World!".to_string()
}

async fn request(State(pool): State<PgPool>) -> Result<Json<Vec<Data>>, StatusCode> {
    let datas = sqlx::query_as::<_, Data>(
        "SELECT name, description, age FROM requests"
    )
    .fetch_all(&pool)
    .await
    
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(datas))
}