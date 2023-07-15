use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub data: T,
    pub success: bool,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gallery {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub datetime: u64,
    #[serde(rename = "type")]
    pub mime_type: String,
    pub animated: bool,
    pub width: u64,
    pub height: u64,
    pub size: u64,
    pub views: u64,
    pub bandwidth: u64,
    pub vote: Option<String>,
    pub favorite: bool,
    pub nsfw: Option<bool>,
    pub section: Option<String>,
    pub account_url: Option<String>,
    pub account_id: Option<i32>,
    pub is_ad: bool,
    pub in_most_viral: bool,
    pub has_sound: bool,
    #[serde(skip)]
    pub tags: Vec<serde_json::Value>,
    pub ad_type: i32,
    pub ad_url: String,
    #[serde(skip)]
    pub edited: String,
    pub in_gallery: bool,
    pub link: String,
    pub comment_count: u64,
    pub favorite_count: u64,
    pub ups: u64,
    pub downs: u64,
    pub points: u64,
    pub score: u64,
    pub is_album: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub datetime: u64,
    pub cover: String,
    pub cover_edited: Option<String>,
    pub cover_width: u64,
    pub cover_height: u64,
    pub account_url: Option<String>,
    pub account_id: Option<u64>,
    pub privacy: String,
    pub layout: String,
    pub views: u64,
    pub link: String,
    pub favorite: bool,
    pub nsfw: bool,
    pub section: Option<String>,
    pub images_count: u64,
    pub in_gallery: bool,
    pub is_ad: bool,
    pub include_album_ads: bool,
    pub is_album: bool,
    pub images: Vec<Image>,

    //kinda useless
    #[serde(skip)]
    pub ad_config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub datetime: u64,

    #[serde(rename = "type")]
    pub mime_type: String,

    pub animated: bool,
    pub width: u64,
    pub height: u64,
    pub size: u64,
    pub views: u64,
    pub bandwidth: u64,
    pub vote: Option<String>,
    pub favorite: bool,
    pub nsfw: Option<bool>,
    pub section: Option<String>,
    pub account_url: Option<String>,
    pub account_id: Option<i32>,
    pub is_ad: bool,
    pub in_most_viral: bool,
    pub has_sound: bool,
    pub tags: Vec<String>,
    pub ad_type: i32,
    pub ad_url: String,

    #[serde(skip)]
    pub edited: String,

    pub in_gallery: bool,
    pub link: String,
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::Album;

    #[allow(dead_code)]
    fn test_deserialization() {
        let paths = vec![
            "data/example_album.json",
            "data/example_album2.json",
        ];
        for path in paths {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            let _: Album = serde_json::from_reader(reader).unwrap();
        }
    }
}
