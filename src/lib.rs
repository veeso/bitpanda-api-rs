//! # bitpanda-api
//!
//! bitpanda-api is a Rust client for the Bitpanda API
//!
//! ## Get started
//!
//! ### Add bitpanda-api to your Cargo.toml 🦀
//!
//! ```toml
//! bitpanda-api = "^0.1"
//! ```
//!
//! Supported features are:
//!
//! - `no-log`: disable logging
//!
//! ## Example
//!
//! ```rust
//! use bitpanda_api::Client;
//! use bitpanda_api::model::AssetClass;
//! use bitpanda_api::model::ohlc::Period;
//!
//! #[tokio::main]
//! async fn main() {
//!
//!     let client = Client::default().x_apikey(env!("X_API_KEY"));
//!
//!     // collect my last 20 trades
//!     client.get_trades_ex(Some(20)).await.expect("failed to collect trades");
//!
//!     // get OHLC for BTC of the last 5 years
//!     let btc = client
//!         .get_assets(AssetClass::Cryptocurrency)
//!         .await
//!         .unwrap()
//!         .into_iter()
//!         .find(|asset| asset.symbol == "BTC")
//!         .unwrap();
//!
//!     let ohlc = client.get_ohlc(Period::FiveYears, &btc.pid, "EUR").await.unwrap();
//! }
//! ```
//!

#![doc(html_playground_url = "https://play.rust-lang.org")]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod api;
pub mod model;

pub use api::{ApiError, Client};
