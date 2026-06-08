use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppError {
    #[allow(dead_code)]
    NotFound(String),
    #[allow(dead_code)]
    Database(String),
    #[allow(dead_code)]
    Cache(String),
    Unauthorized,
    Forbidden(String),
    #[allow(dead_code)]
    MissingContext(String),
    InternalServerError(String),
    #[allow(dead_code)]
    ValidationFailed(String),
    #[allow(dead_code)]
    BadRequest(String),
}

impl AppError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn database_error(message: impl Into<String>) -> Self {
        Self::Database(message.into())
    }

    pub fn unauthorized() -> Self {
        Self::Unauthorized
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden(message.into())
    }

    pub fn validation_failed(message: impl Into<String>) -> Self {
        Self::ValidationFailed(message.into())
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {msg}"),
            AppError::Database(msg) => write!(f, "Database Error: {msg}"),
            AppError::Cache(msg) => write!(f, "Cache Error: {msg}"),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {msg}"),
            AppError::MissingContext(msg) => write!(f, "Missing Context: {msg}"),
            AppError::InternalServerError(msg) => write!(f, "Internal Server Error: {msg}"),
            AppError::ValidationFailed(msg) => write!(f, "Validation Failed: {msg}"),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {msg}"),
        }
    }
}

impl std::error::Error for AppError {}

#[cfg(feature = "server")]
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}
