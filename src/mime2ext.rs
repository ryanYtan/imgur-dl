pub fn mime2ext(mime: &str) -> String {
    match mime {
        "image/jpeg" => "jpeg",
        "image/png" => "png",
        "image/gif" => "gif",
        "video/mp4" => "mp4",
        _ => "",
    }.to_owned()
}
