pub mod get_recent_tracks;

use reqwest::Method;
use serde::Deserialize;

use crate::{
    LastFm, RequestComponent,
    authentication::{Enables, ReadPublic},
    error::{LastFmResult, response::Response},
    page::{Paginated, PaginatedBuilder, PaginationConfig},
    types::user::{Friend, User},
};

#[derive(Deserialize)]
pub struct UserGetInfoResponse {
    user: User,
}

impl<T: RequestComponent + Enables<ReadPublic>> LastFm<T> {
    pub async fn user_get_info(&mut self, user: &str) -> LastFmResult<User> {
        self.request(Method::GET, "user.getinfo")
            .query(&[("user", user)])
            .send()
            .await?
            .json::<Response<UserGetInfoResponse>>()
            .await?
            .into_result()
            .map(|v| v.user)
    }

    pub async fn user_get_friends(&mut self, user: &str) -> LastFmResult<Paginated<Friend>> {
        self.request(Method::GET, "user.getfriends")
            .query(&[("user", user)])
            .paginated::<Friend>("friends", "user", Default::default())
            .await
    }

    pub async fn user_get_friends_with(
        &mut self,
        user: &str,
        config: PaginationConfig,
    ) -> LastFmResult<Paginated<Friend>> {
        self.request(Method::GET, "user.getfriends")
            .query(&[("user", user)])
            .paginated::<Friend>("friends", "user", config)
            .await
    }
}
