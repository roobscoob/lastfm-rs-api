use serde::Deserialize;

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
