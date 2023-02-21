//! # Fiat wallet

use rust_decimal::Decimal;

/// Defines a Bitpanda wallet for fiat currencies
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct FiatWallet {
    pub balance: Decimal,
    pub fiat_id: String,
    pub id: String,
    pub name: String,
    pub pending_transactions_count: usize,
    pub symbol: String,
}
