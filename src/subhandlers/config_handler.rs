use std::path::{PathBuf, Path};

use async_trait::async_trait;
use anyhow::Result;
use lazy_static::lazy_static;

use super::handler_traits::Handler;

pub struct ConfigHandler;

lazy_static! {
    static ref CONFIG_DIR_STR: String = format!("~/.config/").to_owned();
    static ref CONFIG_DIR: PathBuf = Path::new(format!("~/.config/{}/", "ede").as_str()).to_owned();
}

#[async_trait]
impl Handler for ConfigHandler {
    async fn handle(options: &crate::Opt) -> Result<()> {
        match &options.cmd {
            crate::Command::Config { .. } => {
                do_it().await
            },
            _ => panic!("unreachable"),
        }
    }
}

fn create_if_not_exists() {
}


async fn do_it(
) -> Result<()> {

    Ok(())
}
