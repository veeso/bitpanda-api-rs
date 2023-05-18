use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

use crate::model::{InOrOut, TransactionStatus, TransactionType};

/// Defines a Bitpanda transaction for a cryptocurrencies wallet
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct FiatWalletTransaction {
    pub amount: Decimal,
    pub datetime: DateTime<FixedOffset>,
    pub fee: Decimal,
    pub fiat_id: String,
    pub id: String,
    pub in_or_out: InOrOut,
    pub status: TransactionStatus,
    pub to_eur_rate: Decimal,
    pub transaction_type: TransactionType,
    pub user_id: String,
    pub wallet_id: String,
}
