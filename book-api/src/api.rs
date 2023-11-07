use crate::model::SearchedBookItem;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookAPIResponse {
    items: Vec<BookItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BookItem {
    id: String,
    volume_info: BookVolumeInfo,
}

impl Into<SearchedBookItem> for BookItem {
    fn into(self) -> SearchedBookItem {
        let mut isbn_iterator = self.volume_info.industry_identifiers.into_iter();
        let isbn13: Option<String> = isbn_iterator
            .find(|i| i.r#type == "ISBN_13")
            .map(|i| i.identifier.to_owned());
        let isbn10: Option<String> = isbn_iterator
            .find(|i| i.r#type == "ISBN_10")
            .map(|i| i.identifier.to_owned());

        SearchedBookItem {
            id: self.id,
            isbn10,
            isbn13,
            title: self.volume_info.title,
        }
    }
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

pub async fn search_book_by_title(search_query: String) -> anyhow::Result<Vec<SearchedBookItem>> {
    let req_url = format!("https://www.googleapis.com/books/v1/volumes?q={search_query}");
    let res = reqwest::get(req_url).await?;

    let body: BookAPIResponse = res.json().await?;

    let items: Vec<SearchedBookItem> = body
        .items
        .into_iter()
        .map(|item: BookItem| item.into())
        .collect();

    Ok(items)
}
