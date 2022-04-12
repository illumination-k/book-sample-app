use serde_json::Value;
use tonic::{Code, Status};

use crate::book::BookResponse;

fn trim_doublequote<S: ToString>(s: S) -> String {
    s.to_string().trim_matches('"').to_string()
}

pub async fn retrive_bookinfo(isbn: &str) -> Result<BookResponse, Status> {
    let uri = "https://www.googleapis.com/books/v1/volumes"; //?q=isbn:9784043636037;

    let resp = if let Ok(resp) = reqwest::get(format!("{}?q=isbn:{}", uri, isbn)).await {
        resp
    } else {
        return Err(Status::new(tonic::Code::InvalidArgument, "Invalid isbn"));
    };

    let body = resp.text().await.unwrap();
    let json: Value = serde_json::from_str(&body).unwrap();
    let volumeinfo = if let Some(item) = json["items"].as_array().unwrap().get(0) {
        &item["volumeInfo"]
    } else {
        return Err(Status::new(Code::NotFound, "Isbn is not found"));
    };

    let title = trim_doublequote(&volumeinfo["title"]);
    let authors: Vec<String> = volumeinfo["authors"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|v| trim_doublequote(v))
        .collect();
    let publish_date = volumeinfo["publishedDate"].to_string();
    let thumbnail = trim_doublequote(&volumeinfo["imageLinks"]["thumbnail"]);

    Ok(BookResponse {
        isbn: isbn.to_string(),
        title,
        authors,
        publish_date,
        thumbnail,
    })
}
