use std::sync::Arc;

use reqwest::RequestBuilder;

use crate::{
    RequestComponent,
    authentication::{Enables, ReadPublic},
};

#[derive(Clone)]
pub struct PublicAuthentication {
    api_key: Arc<str>,
}

impl Enables<ReadPublic> for PublicAuthentication {}

impl PublicAuthentication {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: Arc::from(api_key),
        }
    }
}

impl RequestComponent for PublicAuthentication {
    fn apply(&self, req: RequestBuilder) -> RequestBuilder {
        req.query(&[("api_key", &self.api_key)])
    }
}
