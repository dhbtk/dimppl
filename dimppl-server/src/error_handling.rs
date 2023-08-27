use std::fmt::{Debug, Display, Formatter};
use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

pub struct AppError(pub anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0)
        ).into_response()
    }
}

impl<E> From<E> for AppError
where
E: Into<anyhow::Error> {
    fn from(value: E) -> Self {
        Self(value.into())
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
