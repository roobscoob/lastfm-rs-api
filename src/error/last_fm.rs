use std::fmt;

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct LastFmErrorResponse {
    pub message: Option<String>,
    pub error: u32,
}

#[derive(Debug)]
pub struct MaybeMsg(pub Option<String>);

impl fmt::Display for MaybeMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.as_deref() {
            Some(s) if !s.is_empty() => write!(f, ": {s:?}"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Error)]
pub enum LastFmError {
    #[error("Invalid service (code 2){message}")]
    InvalidService { message: MaybeMsg },

    #[error("Invalid method (code 3){message}")]
    InvalidMethod { message: MaybeMsg },

    #[error("Authentication failed (code 4){message}")]
    AuthenticationFailed { message: MaybeMsg },

    #[error("Invalid format (code 5){message}")]
    InvalidFormat { message: MaybeMsg },

    #[error("Invalid parameters (code 6){message}")]
    InvalidParameters { message: MaybeMsg },

    #[error("Invalid resource specified (code 7){message}")]
    InvalidResource { message: MaybeMsg },

    #[error("Operation failed (code 8){message}")]
    OperationFailed { message: MaybeMsg },

    #[error("Invalid session key (code 9){message}")]
    InvalidSessionKey { message: MaybeMsg },

    #[error("Invalid API key (code 10){message}")]
    InvalidApiKey { message: MaybeMsg },

    #[error("Service offline (code 11){message}")]
    ServiceOffline { message: MaybeMsg },

    #[error("Invalid method signature (code 13){message}")]
    InvalidSignature { message: MaybeMsg },

    #[error("Temporary error (code 16){message}")]
    TemporaryError { message: MaybeMsg },

    #[error("Suspended API key (code 26){message}")]
    SuspendedApiKey { message: MaybeMsg },

    #[error("Rate limit exceeded (code 29){message}")]
    RateLimitExceeded { message: MaybeMsg },

    #[error("Unknown error (code {code}){message}")]
    Unknown { code: u32, message: MaybeMsg },
}

impl LastFmError {
    pub fn code(&self) -> u32 {
        match self {
            Self::InvalidService { .. } => 2,
            Self::InvalidMethod { .. } => 3,
            Self::AuthenticationFailed { .. } => 4,
            Self::InvalidFormat { .. } => 5,
            Self::InvalidParameters { .. } => 6,
            Self::InvalidResource { .. } => 7,
            Self::OperationFailed { .. } => 8,
            Self::InvalidSessionKey { .. } => 9,
            Self::InvalidApiKey { .. } => 10,
            Self::ServiceOffline { .. } => 11,
            Self::InvalidSignature { .. } => 13,
            Self::TemporaryError { .. } => 16,
            Self::SuspendedApiKey { .. } => 26,
            Self::RateLimitExceeded { .. } => 29,
            Self::Unknown { code, .. } => *code,
        }
    }
}

impl From<LastFmErrorResponse> for LastFmError {
    fn from(value: LastFmErrorResponse) -> Self {
        let msg = MaybeMsg(value.message);
        match value.error {
            2 => Self::InvalidService { message: msg },
            3 => Self::InvalidMethod { message: msg },
            4 => Self::AuthenticationFailed { message: msg },
            5 => Self::InvalidFormat { message: msg },
            6 => Self::InvalidParameters { message: msg },
            7 => Self::InvalidResource { message: msg },
            8 => Self::OperationFailed { message: msg },
            9 => Self::InvalidSessionKey { message: msg },
            10 => Self::InvalidApiKey { message: msg },
            11 => Self::ServiceOffline { message: msg },
            13 => Self::InvalidSignature { message: msg },
            16 => Self::TemporaryError { message: msg },
            26 => Self::SuspendedApiKey { message: msg },
            29 => Self::RateLimitExceeded { message: msg },
            other => Self::Unknown {
                code: other,
                message: msg,
            },
        }
    }
}
