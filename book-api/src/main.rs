use axum::response::IntoResponse;
use serde::Serialize;
use std::net::SocketAddr;

use axum::routing::get;
use axum::{Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/search", get(search));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct BookResponse {
    isbn: String,
    title: String,
    image_url: String,
}

async fn index() -> &'static str {
    "OK"
}

async fn search() -> impl IntoResponse {
    let resp = search_book_by_title().await;
    Json(resp)
}

async fn search_book_by_title() -> BookResponse {
    BookResponse {
        isbn: "xxxxxxxxxx".to_owned(),
        title: "sample title".to_owned(),
        image_url: "https://example.com/images/1.png".to_owned(),
    }
}
