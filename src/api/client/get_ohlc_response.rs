use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

use crate::model::{
    ohlc::{Ohlc, Period},
    OpenHighLowCloseChart,
};

#[derive(Deserialize)]
pub struct GetOhlcResponse {
    data: Vec<Data>,
}

#[derive(Deserialize)]
struct Data {
    attributes: Attributes,
}

#[derive(Deserialize)]
struct Attributes {
    open: Decimal,
    close: Decimal,
    high: Decimal,
    low: Decimal,
    time: Time,
}

#[derive(Deserialize)]
struct Time {
    date_iso8601: DateTime<FixedOffset>,
}

impl GetOhlcResponse {
    pub fn into_ohlc(self, period: Period) -> OpenHighLowCloseChart {
        OpenHighLowCloseChart {
            period,
            chart: self
                .data
                .into_iter()
                .map(|data| Ohlc {
                    open: data.attributes.open,
                    close: data.attributes.close,
                    high: data.attributes.high,
                    low: data.attributes.low,
                    time: data.attributes.time.date_iso8601,
                })
                .collect(),
        }
    }
}
