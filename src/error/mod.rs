pub mod last_fm;
pub mod response;

use thiserror::Error;

use crate::error::last_fm::LastFmError;

pub type LastFmResult<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("{0}")]
    ApiError(#[from] LastFmError),

    #[error("Failed to parse: {0}")]
    ParseError(#[from] serde_json::Error),
}
