use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use lazy_regex::regex;
use rust_decimal::Decimal;

use super::ApiResult;
use crate::{model::crypto_wallet::CryptoWalletTransaction, ApiError};

#[derive(Deserialize)]
pub struct CryptoWalletTxResponse {
    data: Vec<Data>,
    links: Links,
}

impl CryptoWalletTxResponse {
    pub fn next_page(&self) -> Option<usize> {
        if let Some(next) = &self.links.next {
            let rgx = regex!(r"page=([0-9]+)");
            rgx.captures(next).and_then(|captures| {
                captures
                    .get(1)
                    .map(|capture| capture.as_str().parse().unwrap())
            })
        } else {
            None
        }
    }

    pub fn into_transactions(self) -> ApiResult<Vec<CryptoWalletTransaction>> {
        let mut txs = Vec::with_capacity(self.data.len());
        for tx in self.data.into_iter() {
            txs.push(CryptoWalletTransaction::try_from(tx)?)
        }

        Ok(txs)
    }
}

impl TryFrom<Data> for CryptoWalletTransaction {
    type Error = ApiError;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: data.attributes.amount,
            amount_eur: data.attributes.amount_eur,
            confirmations: data.attributes.confirmations,
            cryptocoin_id: data.attributes.cryptocoin_id,
            current_fiat_amount: data.attributes.current_fiat_amount,
            current_fiat_id: data.attributes.current_fiat_id,
            datetime: data.attributes.time.date_iso8601,
            fee: data.attributes.fee,
            id: data.id,
            in_or_out: crate::model::InOrOut::from_str(&data.attributes.in_or_out)?,
            recipient: data.attributes.recipient,
            status: crate::model::TransactionStatus::from_str(&data.attributes.status)?,
            transaction_type: crate::model::TransactionType::from_str(&data.attributes.r#type)?,
            wallet_id: data.attributes.wallet_id,
        })
    }
}

#[derive(Deserialize)]
struct Links {
    next: Option<String>,
}

#[derive(Deserialize)]
struct Data {
    id: String,
    attributes: Attributes,
}

#[derive(Deserialize)]
struct Attributes {
    amount: Decimal,
    recipient: String,
    time: Time,
    confirmations: usize,
    in_or_out: String,
    r#type: String,
    status: String,
    amount_eur: Decimal,
    wallet_id: String,
    cryptocoin_id: String,
    fee: Decimal,
    current_fiat_id: String,
    current_fiat_amount: Decimal,
}

#[derive(Deserialize)]
struct Time {
    date_iso8601: DateTime<FixedOffset>,
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_next_page() {
        let response = CryptoWalletTxResponse {
            data: vec![],
            links: Links {
                next: Some("?page=18&page_size=2".to_string()),
            },
        };

        assert_eq!(response.next_page(), Some(18));

        let response = CryptoWalletTxResponse {
            data: vec![],
            links: Links { next: None },
        };

        assert!(response.next_page().is_none());
    }
}
