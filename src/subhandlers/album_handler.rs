use std::{error::Error, path::{PathBuf, Path}, io::Write};
use async_trait::async_trait;
use crate::{api::ImgurApi, models::album::{ResponseBody, Album, Image}, utility::mime2ext};
use super::handler_traits::Handler;

pub struct AlbumHandler;

#[async_trait]
impl Handler for AlbumHandler {
    async fn handle(options: &crate::Opt) -> Result<(), Box<dyn Error>> {
        match &options.cmd {
            crate::Command::Album { album_hash, output_directory } => {
                do_it(&album_hash, output_directory).await
            },
            _ => {
                panic!("reached unexpected arm");
            }
        }
    }
}

fn get_output_album_folder_name(album: &ResponseBody<Album>) -> String {
    match &album.data.title {
        Some(v) => v,
        None => &album.data.id,
    }.clone()
}

fn get_output_filename(image: &Image, counter: u64) -> String {
    return format!("{:04}-{}", counter, &image.id)
}

async fn do_it(
        album_hash: &str,
        output_directory: &Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let api = ImgurApi::new();
    let result = api.album(&album_hash).await;

    let album = match result {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let outdir = match output_directory {
        Some(v) => v.clone(),
        None => Path::new("./").to_path_buf(),
    };

    //create output album folder
    log::info!("Creating output folder at \"{}\"", &outdir.to_str().unwrap());
    let folder_name = get_output_album_folder_name(&album);
    let folder_path = outdir.join(folder_name);
    match std::fs::create_dir_all(&folder_path) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e)), //TODO
    }

    for (i, image) in album.data.images.iter().enumerate() {
        log::info!("Downloading {}", &image.link);
        let bin_data = match api.get(&image.link).await {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)), // TODO
        };

        let image_filename = get_output_filename(&image, i as u64);
        let image_extension = mime2ext::mime2ext(&image.mime_type);
        let image_path = folder_path
            .join(&image_filename)
            .with_extension(&image_extension);

        log::info!("Saving to \"{}\"...", &image_path.to_str().unwrap());

        let mut file = match std::fs::File::create(&image_path) {
            Ok(v) => v,
            Err(e) => return Err(Box::new(e)) //TODO
        };
        match file.write_all(&bin_data) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e))
        }
    }

    Ok(())
}
