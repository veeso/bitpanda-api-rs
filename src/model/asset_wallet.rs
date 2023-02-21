//! # Asset wallet

use super::AssetClass;

use rust_decimal::Decimal;

/// A Bitpanda wallet for a certain asset
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AssetWallet {
    pub asset_id: String,
    pub asset_symbol: String,
    pub balance: Decimal,
    pub class: AssetClass,
    pub deleted: bool,
    pub id: String,
    pub is_default: bool,
    pub name: String,
}
