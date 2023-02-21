//! # Ohlc
//!
//! Api types for Open-high-low-close chart for assets

use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

/// Open high low close chart type
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct OpenHighLowCloseChart {
    pub symbol: String,
    pub chart: Vec<Ohlc>,
    pub period: Period,
}

/// Defines an entry in the OHLC chart
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Ohlc {
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub open: Decimal,
    pub time: DateTime<FixedOffset>,
}

/// A period which identifies the OHLC chart
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deserialize)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
}
