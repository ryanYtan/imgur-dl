use std::{path::{PathBuf, Path}, io::Write, sync::Arc};
use async_trait::async_trait;
use tokio::task::JoinHandle;
use crate::{api::ImgurApi, utility::{mime2ext, template::{Templater, Formatter}}, models::{Image, Album}};
use super::handler_traits::Handler;
use anyhow::Result;

impl Default for Templater<Album> {
    fn default() -> Self {
        let mut ret = Self::new();
        ret.add_selector("id", |album| Some(album.id.to_owned()));
        ret.add_selector("title", |album| album.title.to_owned());
        ret.add_selector("description", |album| album.description.clone());
        ret.add_selector("datetime", |album| Some(album.datetime.to_string()));
        ret.add_selector("num_imgs", |album| Some(album.images_count.to_string()));
        ret.add_selector("num_views", |album| Some(album.views.to_string()));
        ret.add_selector("section", |album| album.section.clone());
        ret
    }
}

pub struct AlbumHandler;

#[async_trait]
impl Handler for AlbumHandler {
    async fn handle(options: &crate::Opt) -> Result<()> {
        match &options.cmd {
            crate::Command::Album { album_hash, output_directory, output_template } => {
                do_it(&album_hash, output_directory, output_template).await
            },
        }
    }
}

fn get_output_filename(image: &Image, counter: u64) -> String {
    return format!("{:04}-{}", counter, &image.id)
}

async fn do_it(album_hash: &str, output_directory: &Option<PathBuf>, output_template: &str) -> Result<()> {
    log::info!("Processing album {}", &album_hash);

    let api = Arc::new(ImgurApi::new());
    let album = api.album(&album_hash).await?;

    let formatter = Formatter::build(output_template)?;
    let templater = Templater::<Album>::default();


    log::debug!("Retrieved album object\n{:?}", &album);

    let outdir = output_directory
        .clone()
        .or(Some(Path::new("./").to_path_buf()))
        .unwrap();

    //create output album folder
    log::info!("Creating output folder at \"{}\"", &outdir.to_str().unwrap());
    let folder_name = templater.render(&album.data, &formatter)?;
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

    Ok(())
}
