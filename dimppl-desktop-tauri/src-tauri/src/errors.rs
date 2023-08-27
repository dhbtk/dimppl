use std::fmt::{Debug, Display, Formatter};

use serde::Serializer;

pub struct AppError(pub anyhow::Error);

pub type AppResult<T> = std::result::Result<T, AppError>;

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.0.to_string().as_ref())
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
