//! # Asset

/// Describes the asset class
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Deserialize)]
pub enum AssetClass {
    Commodity,
    Cryptocurrency,
    Cryptoindex,
    Etf,
    Metal,
    Stock,
}

impl ToString for AssetClass {
    fn to_string(&self) -> String {
        match self {
            Self::Commodity => "etc",
            Self::Cryptocurrency => "cryptocoin",
            Self::Cryptoindex => "cryptocoin",
            Self::Etf => "etf",
            Self::Metal => "metal",
            Self::Stock => "stock",
        }
        .to_string()
    }
}

/// A bitpanda asset
#[derive(Debug, Clone, Hash, Deserialize, PartialEq, Eq)]
pub struct Asset {
    pub id: String,
    /// Identifier used to get the OHLC
    pub pid: String,
    pub name: String,
    pub symbol: String,
    pub r#type: AssetClass,
}
