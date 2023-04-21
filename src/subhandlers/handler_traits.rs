use std::error::Error;

use async_trait::async_trait;

use crate::Opt;

#[async_trait]
pub trait Handler {
    async fn handle(options: &Opt) -> Result<(), Box<dyn Error>>;
}
