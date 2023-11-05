use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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

struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {}", self.0),
        )
            .into_response()
    }
}
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn index() -> &'static str {
    "OK"
}

async fn search() -> Result<Json<BookResponse>, AppError> {
    let resp = search_book_by_title("データ指向".to_owned()).await?;
    Ok(Json(resp))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookAPIResponse {
    total_items: i64,
    items: Vec<BookItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookItem {
    id: String,
    volume_info: BookVolumeInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookVolumeInfo {
    title: String,
    industry_identifiers: Vec<BookIndustryIdentifier>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookIndustryIdentifier {
    r#type: String,
    identifier: String,
}

async fn search_book_by_title(search_query: String) -> anyhow::Result<BookResponse> {
    let req_url = format!("https://www.googleapis.com/books/v1/volumes?q={search_query}");
    let res = reqwest::get(req_url).await?;

    let body: BookAPIResponse = res.json().await?;
    println!("Body\n{:?}", body);

    let items: Vec<BookItemResponse> = body
        .items
        .into_iter()
        .map(|item: BookItem| {
            let mut isbn_iterator = item.volume_info.industry_identifiers.into_iter();
            let isbn13: Option<String> = isbn_iterator
                .find(|i| i.r#type == "ISBN_13")
                .map(|i| i.identifier.to_owned());
            let isbn10: Option<String> = isbn_iterator
                .find(|i| i.r#type == "ISBN_10")
                .map(|i| i.identifier.to_owned());

            BookItemResponse {
                id: item.id,
                isbn10: isbn10.to_owned(),
                isbn13: isbn13.to_owned(),
                title: item.volume_info.title,
            }
        })
        .collect();

    Ok(BookResponse { items })
}
