use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GcpApiError {
    MissingField {
        field: &'static str,
    },
    InvalidFieldType {
        field: &'static str,
        expected: &'static str,
    },
    InvalidFieldValue {
        field: &'static str,
        message: String,
    },
    Auth {
        message: String,
    },
    Http {
        source: reqwest::Error,
    },
    Serialization {
        source: serde_json::Error,
    },
}

impl fmt::Display for GcpApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GcpApiError::MissingField { field } => {
                write!(f, "missing required field: {}", field)
            }
            GcpApiError::InvalidFieldType { field, expected } => {
                write!(f, "invalid field type for '{}', expected {}", field, expected)
            }
            GcpApiError::InvalidFieldValue { field, message } => {
                write!(f, "invalid value for '{}': {}", field, message)
            }
            GcpApiError::Auth { message } => write!(f, "gcp auth error: {}", message),
            GcpApiError::Http { source } => write!(f, "http error: {}", source),
            GcpApiError::Serialization { source } => write!(f, "serialization error: {}", source),
        }
    }
}

impl Error for GcpApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            GcpApiError::Http { source } => Some(source),
            GcpApiError::Serialization { source } => Some(source),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for GcpApiError {
    fn from(source: reqwest::Error) -> Self {
        GcpApiError::Http { source }
    }
}

impl From<serde_json::Error> for GcpApiError {
    fn from(source: serde_json::Error) -> Self {
        GcpApiError::Serialization { source }
    }
}
