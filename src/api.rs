use reqwest::Client;
use crate::{album::{Album, self}, utility::url_builder::UrlBuilder};

fn build_http_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str("Client-ID ed3b9947e97336f").unwrap()
    );
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub struct ImgurApi {
    client: Client,
    base_url: String,
}

impl ImgurApi {
    fn new() -> Self {
        ImgurApi {
            client: build_http_client(),
            base_url: "https://api.imgur.com/3".to_owned()
        }
    }

    // https://api.imgur.com/3/album/{{album_hash}}/images
    pub async fn album_images(&self, album_hash: &str) -> Result<Album, ()> {
        let url = UrlBuilder::with_url(&self.base_url)
            .subdir("album")
            .subdir(album_hash)
            .subdir("images")
            .build();
        let response =
    }
}
