use chrono::{DateTime, Utc};
use futures::{Stream, StreamExt};
use reqwest::{Method, RequestBuilder};

use crate::{
    LastFm, RequestComponent,
    authentication::{Enables, ReadPublic},
    error::LastFmResult,
    page::{PaginatedBuilder, PaginationConfig},
    types::track::Track,
};

pub struct GetRecentTracks<'a> {
    user: &'a str,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    config: PaginationConfig,
    include_now_playing: bool,
    request: RequestBuilder,
}

impl<T: RequestComponent + Enables<ReadPublic>> LastFm<T> {
    pub fn user_get_recent_tracks<'a>(&mut self, user: &'a str) -> GetRecentTracks<'a> {
        GetRecentTracks {
            user,
            from: Default::default(),
            to: Default::default(),
            config: Default::default(),
            include_now_playing: Default::default(),
            request: self.request(Method::GET, "user.getrecenttracks"),
        }
    }
}

impl<'a> GetRecentTracks<'a> {
    pub fn with_start_date(mut self, from: DateTime<Utc>) -> Self {
        self.from = Some(from);
        self
    }

    pub fn with_end_date(mut self, to: DateTime<Utc>) -> Self {
        self.to = Some(to);
        self
    }

    pub fn with_config(mut self, config: PaginationConfig) -> Self {
        self.config = config;
        self
    }

    pub fn include_now_playing(mut self) -> Self {
        self.include_now_playing = true;
        self
    }

    pub async fn fetch(self) -> LastFmResult<impl Stream<Item = LastFmResult<Track>>> {
        let mut request = self
            .request
            .query(&[("user", self.user), ("extended", "1")]);

        if let Some(from) = self.from {
            request = request.query(&[("from", from.timestamp())]);
        }

        if let Some(to) = self.to {
            request = request.query(&[("to", to.timestamp())]);
        }

        let mut should_emit_now_playing = self.include_now_playing;

        let mut v = request
            .paginated::<Track>("recenttracks", "track", self.config)
            .await?;

        Ok(v.send().filter(move |v| {
            let x = match v.as_ref() {
                Err(_) => true,
                Ok(v) if v.now_playing => {
                    let cap = should_emit_now_playing;
                    should_emit_now_playing = false;
                    cap
                }
                Ok(_) => true,
            };

            async move { x }
        }))
    }
}
