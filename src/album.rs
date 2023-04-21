use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Album {
    data: Vec<Image>,
    success: bool,
    status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Image {
    id: String,
    title: Option<String>,
    description: Option<String>,
    datetime: u64,

    #[serde(rename = "type")]
    mime_type: String,

    animated: bool,
    width: u64,
    height: u64,
    size: u64,
    views: u64,
    bandwidth: u64,
    vote: Option<String>,
    favorite: bool,
    nsfw: Option<bool>,
    section: Option<String>,
    account_url: Option<String>,
    account_id: Option<i32>,
    is_ad: bool,
    in_most_viral: bool,
    has_sound: bool,
    tags: Vec<String>,
    ad_type: i32,
    ad_url: String,

    #[serde(skip)]
    edited: String,

    in_gallery: bool,
    link: String,
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
