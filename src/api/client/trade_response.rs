use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use lazy_regex::regex;
use rust_decimal::Decimal;

use super::ApiResult;

use crate::{
    model::{Trade, TradeStatus, TradeType},
    ApiError,
};

#[derive(Deserialize)]
pub struct TradeResponse {
    data: Vec<Data>,
    links: Links,
}

impl TradeResponse {
    pub fn next_page(&self) -> Option<usize> {
        if let Some(next) = &self.links.next {
            let rgx = regex!(r"page=([0-9]+)");
            rgx.captures(&next)
                .map(|captures| {
                    captures
                        .get(1)
                        .map(|capture| capture.as_str().parse().unwrap())
                })
                .flatten()
        } else {
            None
        }
    }
}

#[derive(Deserialize)]
struct Links {
    next: Option<String>,
}

#[derive(Deserialize)]
struct Data {
    attributes: Attributes,
    id: String,
}

#[derive(Deserialize)]
struct Attributes {
    status: String,
    r#type: String,
    cryptocoin_id: String,
    cryptocoin_symbol: String,
    fiat_id: String,
    amount_fiat: Decimal,
    amount_cryptocoin: Decimal,
    fiat_to_eur_rate: Decimal,
    wallet_id: String,
    fiat_wallet_id: Option<String>,
    related_swap_trade: Option<Box<Data>>,
    time: Time,
    price: Decimal,
}

#[derive(Deserialize)]
struct Time {
    date_iso8601: DateTime<FixedOffset>,
}

impl TradeResponse {
    pub fn into_trades(self) -> ApiResult<Vec<Trade>> {
        let mut trades = Vec::with_capacity(self.data.len());
        for trade in self.data.into_iter() {
            trades.push(Trade::try_from(trade)?)
        }

        Ok(trades)
    }
}

impl TryFrom<Data> for Trade {
    type Error = ApiError;

    fn try_from(trade: Data) -> Result<Self, Self::Error> {
        let related_swap = match trade.attributes.related_swap_trade {
            None => None,
            Some(data) => {
                let trade = Trade::try_from(*data)?;
                Some(Box::new(trade))
            }
        };

        Ok(Trade {
            amount_asset: trade.attributes.amount_cryptocoin,
            amount_fiat: trade.attributes.amount_fiat,
            datetime: trade.attributes.time.date_iso8601,
            fiat_to_eur_rate: trade.attributes.fiat_to_eur_rate,
            fiat_wallet_id: trade.attributes.fiat_wallet_id,
            id_asset: trade.attributes.cryptocoin_id,
            id_fiat: trade.attributes.fiat_id,
            id_wallet: trade.attributes.wallet_id,
            id: trade.id,
            price: trade.attributes.price,
            related_swap_trade: related_swap,
            status: TradeStatus::from_str(&trade.attributes.status)?,
            symbol: trade.attributes.cryptocoin_symbol,
            r#type: TradeType::from_str(&trade.attributes.r#type)?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_next_page() {
        let response = TradeResponse {
            data: vec![],
            links: Links {
                next: Some("?page=18&page_size=2".to_string()),
            },
        };

        assert_eq!(response.next_page(), Some(18));

        let response = TradeResponse {
            data: vec![],
            links: Links { next: None },
        };

        assert!(response.next_page().is_none());
    }
}
