use crate::app_err::Result;
use async_trait::async_trait;
use std::{future::Future, pin::Pin};

use crate::profile::user_profile::UserProfileResponse;

use super::fetcher::Fetcher;

pub struct ProfileFetcher {}

#[async_trait]
impl Fetcher for ProfileFetcher {
    type Output = UserProfileResponse;
    async fn fetch() -> Result<UserProfileResponse> {
        todo!()
    }
}
