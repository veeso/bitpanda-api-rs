//! # Asset

/// Describes the asset class
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
pub enum AssetClass {
    Commodity,
    Cryptocurrency,
    Etf,
    Metal,
    Stock,
}

/// A bitpanda asset
#[derive(Debug, Clone, Hash, Deserialize, PartialEq, Eq)]
pub struct Asset {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub r#type: AssetClass,
}
