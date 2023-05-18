//! # Bitpanda API module

mod client;
mod error;

pub use client::Client;
pub use error::ApiError;

/// Api result
pub type ApiResult<T> = Result<T, ApiError>;
