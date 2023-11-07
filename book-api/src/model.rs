use serde::Serialize;

#[derive(Serialize)]
pub struct SearchedBookItem {
    pub id: String,
    pub isbn10: Option<String>,
    pub isbn13: Option<String>,
    pub title: String,
}
