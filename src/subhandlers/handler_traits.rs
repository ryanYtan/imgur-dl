use async_trait::async_trait;
use crate::Opt;

use anyhow::Result;

#[async_trait]
pub trait Handler {
    async fn handle(options: &Opt) -> Result<()>;
}
