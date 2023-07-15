use async_trait::async_trait;
use anyhow::Result;

use super::handler_traits::Handler;

pub struct UploadHandler;

#[async_trait]
impl Handler for UploadHandler {
    async fn handle(options: &crate::Opt) -> Result<()> {
        match &options.cmd {
            crate::Command::Upload { api_key } => {
                do_it(
                    &api_key
                ).await
            },
            _ => panic!("unreachable"),
        }
    }
}

async fn do_it(
        api_key: &str,
) -> Result<()> {
    Ok(())
}
