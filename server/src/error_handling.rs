use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use std::fmt::{Debug, Display, Formatter};

pub struct AppError(pub anyhow::Error, pub StatusCode);

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.1, format!("Error: {}", self.0)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl AppError {
    pub fn unauthorized() -> Self {
        Self(anyhow::anyhow!("Unauthorized"), StatusCode::UNAUTHORIZED)
    }
}
