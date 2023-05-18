use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
use lazy_regex::regex;
use rust_decimal::Decimal;

use super::ApiResult;
use crate::{model::fiat_wallet::FiatWalletTransaction, ApiError};

#[derive(Deserialize)]
pub struct FiatWalletTxResponse {
    data: Vec<Data>,
    links: Links,
}

impl FiatWalletTxResponse {
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

    pub fn into_transactions(self) -> ApiResult<Vec<FiatWalletTransaction>> {
        let mut txs = Vec::with_capacity(self.data.len());
        for tx in self.data.into_iter() {
            txs.push(FiatWalletTransaction::try_from(tx)?)
        }

        Ok(txs)
    }
}

impl TryFrom<Data> for FiatWalletTransaction {
    type Error = ApiError;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: data.attributes.amount,
            datetime: data.attributes.time.date_iso8601,
            fee: data.attributes.fee,
            id: data.id,
            in_or_out: crate::model::InOrOut::from_str(&data.attributes.in_or_out)?,
            status: crate::model::TransactionStatus::from_str(&data.attributes.status)?,
            transaction_type: crate::model::TransactionType::from_str(&data.attributes.r#type)?,
            wallet_id: data.attributes.fiat_wallet_id,
            fiat_id: data.attributes.fiat_id,
            to_eur_rate: data.attributes.to_eur_rate,
            user_id: data.attributes.user_id,
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
    time: Time,
    fiat_id: String,
    fee: Decimal,
    in_or_out: String,
    r#type: String,
    status: String,
    fiat_wallet_id: String,
    to_eur_rate: Decimal,
    user_id: String,
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
        let response = FiatWalletTxResponse {
            data: vec![],
            links: Links {
                next: Some("?page=18&page_size=2".to_string()),
            },
        };

        assert_eq!(response.next_page(), Some(18));

        let response = FiatWalletTxResponse {
            data: vec![],
            links: Links { next: None },
        };

        assert!(response.next_page().is_none());
    }
}
