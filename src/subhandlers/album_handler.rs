use std::{path::{PathBuf, Path}, io::Write, sync::Arc};
use async_trait::async_trait;
use struct_string_template::{Templater, TemplaterBuilder, Formatter};
use tokio::task::JoinHandle;
use crate::{api::ImgurApi, utility::mime2ext, models::{Image, Album}};
use super::handler_traits::Handler;
use anyhow::Result;

fn create_template() -> Templater<Album> {
    TemplaterBuilder::<Album>::new()
        .with_selector("id", |album| Some(album.id.to_owned()))
        .with_selector("title", |album| album.title.to_owned())
        .with_selector("description", |album| album.description.clone())
        .with_selector("datetime", |album| Some(album.datetime.to_string()))
        .with_selector("num_imgs", |album| Some(album.images_count.to_string()))
        .with_selector("num_views", |album| Some(album.views.to_string()))
        .with_selector("section", |album| album.section.clone())
        .build()
}

pub struct AlbumHandler;

#[async_trait]
impl Handler for AlbumHandler {
    async fn handle(options: &crate::Opt) -> Result<()> {
        match &options.cmd {
            crate::Command::Album { album_hashes, output_directory, output_template } => {
                do_it(&album_hashes, output_directory, output_template).await
            },
        }
    }
}

fn get_output_filename(image: &Image, counter: u64) -> String {
    return format!("{:04}-{}", counter, &image.id)
}

async fn do_it(
        album_hashes: &[String],
        output_directory: &Option<PathBuf>,
        output_template: &str
) -> Result<()> {
    let api = Arc::new(ImgurApi::new());
    let formatter = Formatter::build(output_template)?;
    let templater = create_template();
    let outdir = output_directory
        .clone()
        .or(Some(Path::new("./").to_path_buf()))
        .unwrap();
    for hash in album_hashes {
        log::info!("Processing album {}", &hash);
        let album = api.album(&hash).await?;
        log::debug!("Retrieved album object\n{:?}", &album);

        //create output album folder
        log::info!("Creating output folder at \"{}\"", &outdir.to_str().unwrap());
        let folder_name = templater.renderf(&album.data, &formatter)?;
        let folder_path = Arc::new(outdir.join(&folder_name));
        log::info!("Outputting images to \"{}\"", &folder_name);
        std::fs::create_dir_all(folder_path.as_ref())?;

        let tasks: Vec<JoinHandle<Result<()>>> = album
            .data
            .images
            .into_iter()
            .enumerate()
            .map(|(i, image)| -> JoinHandle<Result<()>> {
                let api = api.clone();
                let folder_path = folder_path.clone();
                tokio::spawn(async move {
                    log::info!("Downloading {}", &image.link);
                    let bin_data = api.get(&image.link).await?;
                    let image_filename = get_output_filename(&image, i as u64);
                    let image_extension = mime2ext::mime2ext(&image.mime_type);
                    let image_path = folder_path
                        .join(&image_filename)
                        .with_extension(&image_extension);
                    let mut file = std::fs::File::create(&image_path)?;
                    file.write_all(&bin_data)?;
                    Ok(())
                })
            })
            .collect();
        futures::future::join_all(tasks).await;
    }
    Ok(())
}
