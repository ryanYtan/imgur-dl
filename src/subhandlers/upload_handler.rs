use async_trait::async_trait;
use anyhow::Result;

use super::handler_traits::Handler;

pub struct UploadHandler;

#[async_trait]
impl Handler for UploadHandler {
    async fn handle(options: &crate::Opt) -> Result<()> {
        todo!("")
    }
}
