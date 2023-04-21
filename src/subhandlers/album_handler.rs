use std::error::Error;

use async_trait::async_trait;

use super::handler_traits::Handler;

pub struct AlbumHandler;

#[async_trait]
impl Handler for AlbumHandler {
    async fn handle(options: &crate::Opt) -> Result<(), Box<dyn Error>> {
        match &options.cmd {
            crate::Command::Album { album_hash } => {
                return Ok(
                    do_it(&album_hash).await?
                )
            }
            _ => {
                panic!("reached unexpected arm");
            }
        };
    }
}

async fn do_it(
    album_hash: &str,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}
