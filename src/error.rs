use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "rarible")]
pub use rarible::RaribleApiError;

/// Describes API errors
#[derive(Debug)]
pub enum ApiError {
    Unauthorized,
    Other(u16),
    OpenSeaApiError(OpenSeaApiError),
    #[cfg(feature = "rarible")]
    RaribleServerError(RaribleApiError),
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "Unauthorized request to API"),
            ApiError::Other(s) => write!(f, "API reported error code {}", s),
            ApiError::OpenSeaApiError(err) => {
                write!(f, "OpenSea Server success {}", err.success)
            }
            #[cfg(feature = "rarible")]
            ApiError::RaribleServerError(err) => err.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenSeaApiError {
    pub success: bool,
}
impl From<OpenSeaApiError> for ApiError {
    fn from(err: OpenSeaApiError) -> Self {
        ApiError::OpenSeaApiError(err)
    }
}

#[cfg(feature = "rarible")]
mod rarible {
    use super::ApiError;
    use serde::{Deserialize, Serialize};
    use std::fmt;

    impl From<RaribleApiError> for ApiError {
        fn from(err: RaribleApiError) -> Self {
            ApiError::RaribleServerError(err)
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct RaribleApiError {
        #[serde(rename = "status")]
        pub status: i32,
        #[serde(rename = "code")]
        pub code: String,
        #[serde(rename = "message")]
        pub message: String,
    }
    impl fmt::Display for RaribleApiError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "status: `{}`, ", self.status)?;
            write!(f, "code: `{}`, ", self.code)?;
            write!(f, "message: `{}`", self.message)
        }
    }
}
