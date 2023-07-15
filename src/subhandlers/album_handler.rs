use std::{path::{PathBuf, Path}, io::Write, sync::Arc};
use async_trait::async_trait;
use either::Either;
use struct_string_template::{Templater, TemplaterBuilder, Formatter};
use tokio::task::JoinHandle;
use crate::{api::ImgurApi, models::{Image, Album, Gallery, ResponseBody}, mime2ext};
use super::handler_traits::Handler;
use anyhow::{Result, bail};

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
            crate::Command::Album(album_command) => {
                do_it(&album_command).await
            },
            _ => panic!("unreachable"),
        }
    }
}

fn get_output_filename(image: &Image, counter: u64) -> String {
    return format!("{:04}-{}", counter, &image.id)
}

async fn do_it(
    album_command: &crate::AlbumCommand,
) -> Result<()> {
    match album_command {
        crate::AlbumCommand::Download(opts) => {
            handle_download(
                &opts.output_directory,
                &opts.output_template,
                &opts.album_hashes
            ).await
        },
        crate::AlbumCommand::Info(opts) => {
            handle_info(
                &opts.output_template,
                &opts.album_hash
            ).await
        },
    }
}

async fn handle_download(
        output_directory: &Option<PathBuf>,
        output_template: &str,
        album_hashes: &[String]
) -> Result<()> {
    let api = Arc::new(ImgurApi::new());
    let templater = create_template();
    let formatter = Formatter::build(output_template)?;
    let outdir = output_directory
        .clone()
        .or(Some(Path::new("./").to_path_buf()))
        .unwrap();

    for hash in album_hashes {
        log::info!("Processing album {}", &hash);

        let album_or_gallery = try_download(api.clone(), hash).await?;

        match album_or_gallery {
            Either::Left(album) => {
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
            },
            Either::Right(gallery) => {
                log::debug!("Retrieved gallery object\n{:?}", &gallery);
                let image_extension = mime2ext::mime2ext(&gallery.data.mime_type);
                let image_path = outdir
                    .join(&gallery.data.id)
                    .with_extension(&image_extension);
                let link = &gallery.data.link;
                log::info!("Downloading {}", &link);
                let bin_data = api.get(&link).await?;
                let mut file = std::fs::File::create(&image_path)?;
                file.write_all(&bin_data)?;
            },
        }

    }
    Ok(())
}

async fn try_download(
        api: Arc<ImgurApi>,
        hash: &str
) -> Result<Either<ResponseBody<Album>, ResponseBody<Gallery>>> {
    let album = api.album(hash).await;
    if album.is_ok() {
        return Ok(Either::Left(album.ok().unwrap()));
    }
    let gallery = api.gallery(hash).await;
    if gallery.is_ok() {
        return Ok(Either::Right(gallery.ok().unwrap()));
    }
    bail!("unable to download {} to either an album or gallery", hash);
}

async fn handle_info(
        output_template: &str,
        album_hash: &str
) -> Result<()> {
    let api = Arc::new(ImgurApi::new());
    let templater = create_template();
    let formatter = Formatter::build(output_template)?;
    let album = api.album(&album_hash).await?;
    let sout = templater.renderf(&album.data, &formatter)?;
    println!("{}", &sout);
    Ok(())
}
