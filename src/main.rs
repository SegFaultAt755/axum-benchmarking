use serde::Serialize;
use axum::{
    Router,
    Json,
    routing::get
};

#[derive(Serialize)]
struct Data {
    name: String,
    description: String,
    age: i32
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/greeting", get(greeting))
        .route("/request", get(request));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn greeting() -> String {
    "Hello, World!".to_string()
}

async fn request() -> Json<Data> {
    let data = Data {
        name: "alice".to_string(),
        description: "some description".to_string(),
        age: 24
    };

    Json(data)
}