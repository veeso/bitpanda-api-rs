use chrono::{DateTime, FixedOffset};
use rust_decimal::Decimal;

use crate::model::{InOrOut, TransactionStatus, TransactionType};

/// Defines a Bitpanda transaction for a cryptocurrencies wallet
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct CryptoWalletTransaction {
    pub amount_eur: Decimal,
    pub amount: Decimal,
    pub confirmations: usize,
    pub cryptocoin_id: String,
    pub current_fiat_amount: Decimal,
    pub current_fiat_id: String,
    pub datetime: DateTime<FixedOffset>,
    pub fee: Decimal,
    pub id: String,
    pub in_or_out: InOrOut,
    pub recipient: String,
    pub status: TransactionStatus,
    pub transaction_type: TransactionType,
    pub wallet_id: String,
}
