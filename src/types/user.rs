use std::sync::Arc;
use std::time::SystemTime;

use super::bool_from_strnum;
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Deserializer};
use serde_with::serde_as;
use serde_with::{DisplayFromStr, TimestampSeconds};

use crate::types::image::Image;

#[serde_as]
#[derive(Deserialize)]
pub struct User {
    pub name: Arc<str>,

    #[serde_as(as = "DisplayFromStr")]
    pub age: u32,

    #[serde(deserialize_with = "bool_from_strnum")]
    pub subscriber: bool,

    pub realname: Arc<str>,

    #[serde(deserialize_with = "bool_from_strnum")]
    pub bootstrap: bool,

    #[serde_as(as = "DisplayFromStr")]
    pub playcount: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub artist_count: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub playlists: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub track_count: u32,

    #[serde_as(as = "DisplayFromStr")]
    pub album_count: u32,

    pub image: Vec<Image>,
    pub country: Arc<str>,
    pub gender: Arc<str>,
    pub url: Url,
    pub kind: Arc<str>,

    #[serde(deserialize_with = "de_registered")]
    pub registered: DateTime<Utc>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct Friend {
    pub name: Arc<str>,

    #[serde(deserialize_with = "bool_from_strnum")]
    pub subscriber: bool,

    pub realname: Arc<str>,

    #[serde(deserialize_with = "bool_from_strnum")]
    pub bootstrap: bool,

    pub image: Vec<Image>,
    pub country: Arc<str>,
    pub url: Url,

    #[serde(deserialize_with = "de_registered")]
    pub registered: DateTime<Utc>,
}

#[serde_as]
#[derive(Deserialize)]
struct RegisteredHelper {
    #[serde_as(as = "TimestampSeconds<i64>")]
    unixtime: SystemTime,
}

fn de_registered<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
    let helper = RegisteredHelper::deserialize(d)?;
    Ok(DateTime::from(helper.unixtime))
}
