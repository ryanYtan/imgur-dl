use reqwest::{Client, StatusCode};
use snafu::{Whatever, whatever, Snafu};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};

use crate::{utility::url_builder::UrlBuilder, models::album::{ResponseBody, Album, Image}};

#[derive(Debug, Snafu)]
pub enum ClientError {
    DoesNotExist,
    FailedRequest,
}

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

async fn http_get(client: &Client, url: &str) -> Result<Vec<u8>, ClientError> {
    let http_get_impl = || async {
        let request = client.get(url);
        let response = request.send().await.unwrap(); //assume it passes

        match response.status() {
            StatusCode::OK => {
                Ok(response.bytes().await.unwrap().to_vec())
            },
            StatusCode::NOT_FOUND => {
                Err(ClientError::DoesNotExist)
            },
            _ => {
                panic!("unexpected status code {} for url={}", &response.status(), &url)
            }
        }
    };

    let retry_strategy = ExponentialBackoff::from_millis(10)
        .map(jitter)
        .take(3);

    Retry::spawn(retry_strategy, http_get_impl).await
}

pub struct ImgurApi {
    client: Client,
    base_url: String,
}

impl ImgurApi {
    pub fn new() -> Self {
        ImgurApi {
            client: build_http_client(),
            base_url: "https://api.imgur.com/3".to_owned()
        }
    }

    pub async fn get(&self, url: &str) -> Result<Vec<u8>, ClientError> {
        http_get(&self.client, &url).await
    }

    // https://api.imgur.com/3/album/{{album_hash}}
    pub async fn album(&self, album_hash: &str) -> Result<ResponseBody<Album>, ClientError> {
        let url = UrlBuilder::with_url(&self.base_url)
            .subdir("album")
            .subdir(album_hash)
            .build();
        let result = http_get(&self.client, &url).await?;
        Ok(serde_json::from_slice(&result).unwrap())
    }

    // https://api.imgur.com/3/album/{{album_hash}}/images
    pub async fn album_images(&self, album_hash: &str) -> Result<ResponseBody<Vec<Image>>, ClientError> {
        let url = UrlBuilder::with_url(&self.base_url)
            .subdir("album")
            .subdir(album_hash)
            .subdir("images")
            .build();
        let result = http_get(&self.client, &url).await?;
        Ok(serde_json::from_slice(&result).unwrap())
    }
}
