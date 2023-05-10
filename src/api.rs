use reqwest::{Client, StatusCode, header::{HeaderValue, HeaderMap}};
use snafu::{Snafu};
use tokio_retry::{strategy::{jitter, ExponentialBackoff}, Retry};
use lazy_static::lazy_static;
use crate::{utility::url_builder::UrlBuilder, models::{ResponseBody, Album, Image}};

lazy_static! {
    static ref IMGUR_AUTHORIZATION_HEADER: HeaderMap = (|| {
        let mut headers = HeaderMap::new();
        headers.append(http::header::AUTHORIZATION, HeaderValue::from_str("Client-ID ed3b9947e97336f").unwrap());
        headers
    })();
}

#[derive(Debug, Snafu)]
pub enum ClientError {
    DoesNotExist,
    FailedRequest { msg: String },
}

fn build_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .build()
        .unwrap()
}

async fn http_get(client: &Client, url: &str, headers: &HeaderMap) -> Result<Vec<u8>, ClientError> {
    let http_get_impl = || async {
        let request = client
            .get(url);
        let response = request
            .headers(headers.clone())
            .send()
            .await
            .unwrap(); //assume it passes

        match response.status() {
            StatusCode::OK => {
                Ok(response.bytes().await.unwrap().to_vec())
            },
            StatusCode::NOT_FOUND => {
                Err(ClientError::DoesNotExist)
            },
            _ => {
                Err(ClientError::FailedRequest {
                    msg: format!("unexpected status code {} for url={}", &response.status(), &url),
                })
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
        let headers = HeaderMap::new();
        http_get(&self.client, &url, &headers).await
    }

    pub async fn get_header(&self, url: &str, headers: &HeaderMap) -> Result<Vec<u8>, ClientError> {
        http_get(&self.client, &url, &headers).await
    }

    // https://api.imgur.com/3/album/{{album_hash}}
    pub async fn album(&self, album_hash: &str) -> Result<ResponseBody<Album>, ClientError> {
        let url = UrlBuilder::with_url(&self.base_url)
            .subdir("album")
            .subdir(album_hash)
            .build();
        let result = self.get_header(&url, &IMGUR_AUTHORIZATION_HEADER).await?;
        Ok(serde_json::from_slice(&result).unwrap())
    }

    // https://api.imgur.com/3/album/{{album_hash}}/images
    pub async fn album_images(&self, album_hash: &str) -> Result<ResponseBody<Vec<Image>>, ClientError> {
        let url = UrlBuilder::with_url(&self.base_url)
            .subdir("album")
            .subdir(album_hash)
            .subdir("images")
            .build();
        let result = self.get_header(&url, &IMGUR_AUTHORIZATION_HEADER).await?;
        Ok(serde_json::from_slice(&result).unwrap())
    }
}
