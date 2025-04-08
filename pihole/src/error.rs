use crate::data::PiHoleError;
use std::error::Error;

#[derive(Debug)]
pub enum PiHoleApiError {
    AuthenticationRequired,
    HttpApiError(PiHoleError),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    SerdeJsonBiError(serde_json::Error, serde_json::Error),
}

impl Error for PiHoleApiError {}

// Implement the Error trait for the custom error type
impl std::fmt::Display for PiHoleApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Implement the Display trait for the custom error type
        match self {
            PiHoleApiError::AuthenticationRequired => {
                write!(f, "Unauthorized: Authentication required.")
            }
            PiHoleApiError::HttpApiError(e) => write!(
                f,
                "PiHole API error: (key: {}, message: {})",
                e.error.key, e.error.message
            ),
            PiHoleApiError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            PiHoleApiError::SerdeJsonError(e) => write!(f, "serde_json error: {}", e),
            PiHoleApiError::SerdeJsonBiError(outer, inner) => {
                write!(f, "serde_json error: (outer: {}, inner: {})", outer, inner)
            }
        }
    }
}

impl From<PiHoleError> for PiHoleApiError {
    fn from(err: PiHoleError) -> Self {
        PiHoleApiError::HttpApiError(err)
    }
}

impl From<reqwest::Error> for PiHoleApiError {
    fn from(err: reqwest::Error) -> Self {
        PiHoleApiError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for PiHoleApiError {
    fn from(err: serde_json::Error) -> Self {
        PiHoleApiError::SerdeJsonError(err)
    }
}
