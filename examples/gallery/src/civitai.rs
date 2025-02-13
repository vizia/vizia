use bytes::Bytes;
use serde::Deserialize;
use vizia::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageData {
    pub id: Id,
    pub url: String,
}

pub async fn list() -> Result<Vec<ImageData>, reqwest::Error> {
    let client = reqwest::Client::new();

    #[derive(Deserialize)]
    struct Response {
        items: Vec<ImageData>,
    }

    let response: Response = client
        .get("https://civitai.com/api/v1/images")
        .query(&[
            ("sort", "Most Reactions"),
            ("period", "AllTime"),
            ("nsfw", "None"),
            ("limit", &LIMIT.to_string()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(response.items)
}

pub async fn download(url: String, size: Size) -> Result<Bytes, reqwest::Error> {
    let client = reqwest::Client::new();

    let bytes = client
        .get(match size {
            Size::Original => url,
            Size::Thumbnail => url
                .split("/")
                .map(|part| if part.starts_with("width=") { "width=640" } else { part })
                .collect::<Vec<_>>()
                .join("/"),
        })
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    Ok(bytes)
}

pub const LIMIT: usize = 99;

#[derive(Data, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Original,
    Thumbnail,
}
