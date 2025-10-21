pub mod attributes;
pub mod serde;

use std::{marker::PhantomData, sync::Arc};

use ::serde::de::{DeserializeOwned, DeserializeSeed};
use futures::{Stream, stream};
use reqwest::RequestBuilder;
use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::{
    error::LastFmResult,
    page::{attributes::Attributes, serde::PageSeed},
};

pub struct Page<T> {
    pub attr: Attributes,
    pub items: Vec<T>,
}

#[derive(Clone, Copy)]
pub struct PaginationConfig {
    pub page_size: usize,
}

impl Default for PaginationConfig {
    fn default() -> Self {
        PaginationConfig { page_size: 50 }
    }
}

pub struct Paginated<T: DeserializeOwned> {
    request: RequestBuilder,
    root: Arc<str>,
    content: Arc<str>,
    phantom: PhantomData<fn() -> T>,
    cache: Option<AllocRingBuffer<T>>,
    attributes: Option<Attributes>,
}

pub trait PaginatedBuilder {
    fn paginated<T: DeserializeOwned>(
        self,
        root: &str,
        content: &str,
        config: PaginationConfig,
    ) -> impl Future<Output = LastFmResult<Paginated<T>>>;
}

impl PaginatedBuilder for RequestBuilder {
    async fn paginated<T: DeserializeOwned>(
        self,
        root: &str,
        content: &str,
        config: PaginationConfig,
    ) -> LastFmResult<Paginated<T>> {
        let mut pg = Paginated {
            request: self.query(&[("limit", config.page_size)]),
            root: Arc::from(root),
            content: Arc::from(content),
            phantom: PhantomData::default(),
            cache: None,
            attributes: None,
        };

        pg.attributes = Some(pg.send_with(1).await?);

        Ok(pg)
    }
}

impl<T: DeserializeOwned> Paginated<T> {
    async fn send_with(&mut self, page: usize) -> LastFmResult<Attributes> {
        let bytes = self
            .request
            .try_clone()
            .unwrap()
            .query(&[("page", page)])
            .send()
            .await?
            .bytes()
            .await?;

        let mut de = serde_json::Deserializer::from_slice(&bytes);

        let page = PageSeed::<T>::new(&self.root, &self.content).deserialize(&mut de)?;

        if self.cache.as_ref().is_some_and(|c| c.len() != 0) {
            panic!("Unexpected state");
        }

        if page.items.len() != 0 {
            self.cache = Some(AllocRingBuffer::from(page.items));
        } else {
            self.cache = Some(AllocRingBuffer::new(1))
        }

        Ok(page.attr)
    }

    pub fn send(&mut self) -> impl Stream<Item = LastFmResult<T>> + use<T> {
        struct StreamHead<T: DeserializeOwned> {
            pg: Paginated<T>,
            page: usize,
        }

        let cache = self.cache.take();
        let page = if cache.is_some() { 2 } else { 1 };

        stream::try_unfold(
            StreamHead {
                page,
                pg: Paginated {
                    request: self
                        .request
                        .try_clone()
                        .expect("To be able to clone the request"),
                    root: self.root.clone(),
                    content: self.content.clone(),
                    phantom: self.phantom.clone(),
                    cache: cache,
                    attributes: self.attributes.clone(),
                },
            },
            |mut st| async move {
                if let Some(v) = st.pg.cache.as_mut().and_then(|v| v.dequeue()) {
                    return Ok(Some((v, st)));
                }

                let next_page = st.page;
                st.page += 1;

                let v = st.pg.send_with(next_page).await?;

                println!("{:?}", v);
                if v.page > v.total_pages {
                    return Ok(None);
                }

                Ok(st
                    .pg
                    .cache
                    .as_mut()
                    .and_then(|v| v.dequeue())
                    .map(|v| (v, st)))
            },
        )
    }
}
