pub fn mime2ext(mime: &str) -> String {
    match mime {
        "image/jpeg" => "jpeg",
        "image/png" => "png",
        "image/gif" => "gif",
        _ => "",
    }.to_owned()
}
