use async_trait::async_trait;
use anyhow::Result;

use super::handler_traits::Handler;

pub struct ConfigHandler;

#[async_trait]
impl Handler for ConfigHandler {
    async fn handle(options: &crate::Opt) -> Result<()> {
        todo!("")
    }
}
