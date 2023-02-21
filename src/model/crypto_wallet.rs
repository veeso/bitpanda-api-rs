//! # Crypto wallet

use rust_decimal::Decimal;

/// Defines a Bitpanda wallet for cryptocurrencies
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct CryptoWallet {
    pub balance: Decimal,
    pub cryptocoin_id: String,
    pub deleted: bool,
    pub id: String,
    pub is_default: bool,
    pub name: String,
    pub pending_transactions_count: usize,
    pub symbol: String,
}
