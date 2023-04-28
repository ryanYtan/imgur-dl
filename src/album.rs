use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    pub data: Vec<Image>,
    pub success: bool,
    pub status: i32,
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

    #[test]
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
