use async_trait::async_trait;
use crate::app_err::Result;

#[async_trait]
pub trait Fetcher {
    type Output;
    async fn fetch() -> Result<Self::Output>;
}
