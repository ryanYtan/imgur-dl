use anyhow::{Result, bail};

pub fn mime2ext(mime: &str) -> String {
    match mime {
        "image/jpeg" => "jpeg",
        "image/png" => "png",
        "image/gif" => "gif",
        "video/mp4" => "mp4",
        _ => "",
    }.to_owned()
}

pub async fn try_deserialize<'a, T>(data: &'a [u8]) -> Result<T>
    where T: Deserialize<'a>
{
    match serde_json::from_slice(&data) {
        Ok(v) => Ok(v),
        Err(_) => bail!(""), //TODO
    }
}
