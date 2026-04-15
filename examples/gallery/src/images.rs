use bytes::Bytes;
use serde::Deserialize;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    #[default]
    Loading,
    Loaded,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    pub id: Id,
    /// Base URL: `https://picsum.photos/id/{id}` — append `/{w}/{h}` to fetch.
    pub url: String,
}

pub async fn list() -> Result<Vec<ImageData>, reqwest::Error> {
    #[derive(Deserialize)]
    struct PicsumItem {
        id: String,
    }

    let client = reqwest::Client::new();

    let items: Vec<PicsumItem> = client
        .get("https://picsum.photos/v2/list")
        .query(&[("limit", &LIMIT.to_string())])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(items
        .into_iter()
        .filter_map(|item| {
            item.id.parse::<u32>().ok().map(|n| ImageData {
                id: Id(n),
                url: format!("https://picsum.photos/id/{n}"),
            })
        })
        .collect())
}

pub async fn download(url: String, size: Size) -> Result<Bytes, reqwest::Error> {
    let client = reqwest::Client::new();

    let url = match size {
        Size::Thumbnail => format!("{url}/640/410"),
        Size::Original => format!("{url}/1920/1200"),
    };

    let bytes = client.get(url).send().await?.error_for_status()?.bytes().await?;

    Ok(bytes)
}

pub const LIMIT: usize = 99;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Id(pub u32);

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Original,
    Thumbnail,
}
