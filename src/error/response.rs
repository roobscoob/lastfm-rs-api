use std::marker::PhantomData;

use serde::{
    Deserialize, Deserializer,
    de::{self, DeserializeSeed},
};

use crate::error::last_fm::LastFmErrorResponse;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Ok(T),
    Err(LastFmErrorResponse),
}

impl<T> Response<T> {
    /// Convert into a standard Result, mapping the API error payload to your typed error.
    pub fn into_result(self) -> Result<T, super::Error> {
        match self {
            Response::Ok(value) => Ok(value),
            Response::Err(err) => Err(super::Error::ApiError(err.into())),
        }
    }
}

pub struct ResponseSeed<T, S: for<'de> DeserializeSeed<'de, Value = T>> {
    ok_seed: S,
    _ph: PhantomData<fn() -> T>,
}

impl<S: for<'de> DeserializeSeed<'de, Value = T>, T> ResponseSeed<T, S> {
    pub fn new(ok_seed: S) -> Self {
        Self {
            ok_seed,
            _ph: PhantomData::default(),
        }
    }
}

impl<'da, S: for<'de> DeserializeSeed<'de, Value = T>, T> DeserializeSeed<'da>
    for ResponseSeed<T, S>
{
    type Value = Response<T>;

    fn deserialize<D: Deserializer<'da>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        let content = serde_content::Value::deserialize(deserializer)?;

        let err_de = serde_content::Deserializer::new(content.clone());
        if let Ok(err_value) = LastFmErrorResponse::deserialize(err_de) {
            return Ok(Response::Err(err_value));
        }

        self.ok_seed
            .deserialize(serde_content::Deserializer::new(content))
            .map(Response::Ok)
            .map_err(de::Error::custom)
    }
}
