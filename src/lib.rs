use reqwest::{Method, RequestBuilder};

pub mod authentication;
pub mod error;
pub mod page;
pub mod request;
pub mod types;

pub trait RequestComponent: Send + Sync + Clone {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder;
}

impl RequestComponent for () {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder {
        req
    }
}

#[derive(Clone)]
pub struct LastFm<T: RequestComponent> {
    client: reqwest::Client,
    authentication_component: T,
}

impl LastFm<()> {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
            authentication_component: (),
        }
    }
}

impl<T: RequestComponent> LastFm<T> {
    pub fn with_client(mut self, c: reqwest::Client) -> Self {
        self.client = c;
        self
    }

    pub fn with_authentication<C: RequestComponent + 'static>(self, c: C) -> LastFm<C> {
        LastFm {
            client: self.client,
            authentication_component: c,
        }
    }

    pub fn request(&mut self, http_method: Method, lastfm_method: &str) -> RequestBuilder {
        let mut request = self
            .client
            .request(http_method, "https://ws.audioscrobbler.com/2.0/")
            .query(&[("method", lastfm_method), ("format", "json")]);

        request = self.authentication_component.apply(request);

        request
    }
}
