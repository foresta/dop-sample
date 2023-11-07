mod api;
mod error;
mod model;

use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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
    items: Vec<BookItemResponse>,
}

#[derive(Serialize)]
struct BookItemResponse {
    id: String,
    isbn10: Option<String>,
    isbn13: Option<String>,
    title: String,
}

impl From<model::SearchedBookItem> for BookItemResponse {
    fn from(model: model::SearchedBookItem) -> BookItemResponse {
        BookItemResponse {
            id: model.id,
            isbn10: model.isbn10,
            isbn13: model.isbn13,
            title: model.title,
        }
    }
}

async fn index() -> &'static str {
    "OK"
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

async fn search(query: Query<SearchQuery>) -> Result<Json<BookResponse>, error::AppError> {
    println!("search_query: {}", query.q);

    let searched_books = api::search_book_by_title(query.q.to_owned()).await?;
    let resp = BookResponse {
        items: searched_books
            .into_iter()
            .map(|book| BookItemResponse::from(book))
            .collect(),
    };
    Ok(Json(resp))
}
