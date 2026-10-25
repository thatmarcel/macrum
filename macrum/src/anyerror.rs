use axum::response::{IntoResponse, Response};
use http::StatusCode;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct AnyError {
    inner: Arc<anyhow::Error>
}

impl Debug for AnyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

impl Display for AnyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl IntoResponse for AnyError {
    fn into_response(self) -> Response {
        eprintln!("{:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error.\n").into_response()
    }
}

impl<T> From<T> for AnyError where T: Into<anyhow::Error> + 'static, {
    fn from(error: T) -> AnyError {
        AnyError {
            inner: Arc::new(error.into())
        }
    }
}

pub struct EmptyError {}

impl From<()> for EmptyError {
    fn from(_: ()) -> Self {
        Self {}
    }
}

impl Into<()> for EmptyError {
    fn into(self) -> () {}
}

impl Debug for EmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::write(f, format_args!("EmptyError"))
    }
}

impl Display for EmptyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::write(f, format_args!("EmptyError"))
    }
}

impl std::error::Error for EmptyError {}