use std::sync::Arc;

use super::bool_from_strnum;
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use serde_with::serde_as;

use crate::types::image::Image;

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct Track {
    pub artist: TrackArtist,
    pub album: TrackAlbum,
    pub name: Arc<str>,

    pub url: Url,

    pub image: Vec<Image>,

    #[serde(default, rename = "mbid", deserialize_with = "de_opt_arcstr_empty")]
    pub musicbrainz_id: Option<Arc<str>>,

    #[serde(deserialize_with = "bool_from_strnum")]
    pub streamable: bool,

    #[serde(deserialize_with = "bool_from_strnum")]
    pub loved: bool,

    #[serde(default, deserialize_with = "de_played_at")]
    pub played_at: Option<DateTime<Utc>>,

    #[serde(default, rename = "@attr", deserialize_with = "de_now_playing")]
    pub now_playing: bool,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
#[serde()]
pub struct TrackArtist {
    #[serde(default)]
    pub url: Option<Url>,

    pub name: Arc<str>,
    pub image: Vec<Image>,

    #[serde(default, rename = "mbid", deserialize_with = "de_opt_arcstr_empty")]
    pub musicbrainz_id: Option<Arc<str>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct TrackAlbum {
    #[serde(default, rename = "mbid", deserialize_with = "de_opt_arcstr_empty")]
    pub musicbrainz_id: Option<Arc<str>>,

    #[serde(rename = "#text")]
    pub title: Arc<str>,
}

fn de_opt_arcstr_empty<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Arc<str>>, D::Error> {
    let opt = Option::<String>::deserialize(d)?;
    Ok(opt.and_then(|s| {
        let s = s.trim();
        (!s.is_empty()).then(|| Arc::<str>::from(s.to_owned()))
    }))
}

fn de_played_at<'de, D: Deserializer<'de>>(d: D) -> Result<Option<DateTime<Utc>>, D::Error> {
    let v = Option::<Value>::deserialize(d)?;
    if let Some(Value::Object(map)) = v {
        if let Some(uts_val) = map.get("uts") {
            let uts_i64 = match uts_val {
                Value::String(s) => s.parse::<i64>().map_err(serde::de::Error::custom)?,
                Value::Number(n) => n
                    .as_i64()
                    .ok_or_else(|| serde::de::Error::custom("uts not an i64"))?,
                _ => return Ok(None),
            };
            return Ok(DateTime::from_timestamp(uts_i64, 0));
        }
    }
    Ok(None)
}

fn de_now_playing<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    let v = Option::<Value>::deserialize(d)?;
    if let Some(Value::Object(map)) = v {
        if let Some(Value::String(s)) = map.get("nowplaying") {
            return Ok(matches!(
                s.trim().to_ascii_lowercase().as_str(),
                "true" | "t" | "yes" | "y"
            ));
        }
        if let Some(Value::Bool(b)) = map.get("nowplaying") {
            return Ok(*b);
        }
    }
    Ok(false)
}
