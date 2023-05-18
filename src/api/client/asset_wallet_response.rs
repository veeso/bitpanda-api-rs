use rust_decimal::Decimal;

use crate::model::{AssetClass, AssetWallet};

#[derive(Deserialize)]
pub struct AssetWalletResponse {
    data: Data,
}

#[derive(Deserialize)]
pub struct Data {
    attributes: Attributes,
}

#[derive(Deserialize)]
pub struct Attributes {
    cryptocoin: WalletCollection,
    commodity: CommodityWalletCollection,
    index: IndexWalletCollection,
    security: SecurityWalletCollection,
}

#[derive(Deserialize)]
pub struct WalletCollection {
    attributes: WalletCollectionAttributes,
}

#[derive(Deserialize)]
pub struct WalletCollectionAttributes {
    wallets: Vec<Wallet>,
}

#[derive(Deserialize)]
pub struct Wallet {
    id: String,
    attributes: WalletAttributes,
}

#[derive(Deserialize)]
pub struct WalletAttributes {
    cryptocoin_id: String,
    cryptocoin_symbol: String,
    balance: Decimal,
    is_default: bool,
    name: String,
    deleted: bool,
}

#[derive(Deserialize)]
pub struct CommodityWalletCollection {
    metal: WalletCollection,
}

#[derive(Deserialize)]
pub struct IndexWalletCollection {
    index: WalletCollection,
}

#[derive(Deserialize)]
pub struct SecurityWalletCollection {
    etc: WalletCollection,
    etf: WalletCollection,
    stock: WalletCollection,
}

impl AssetWalletResponse {
    pub fn into_asset_wallets(self) -> Vec<AssetWallet> {
        let mut wallets = Vec::new();

        let attr = self.data.attributes;
        for wallet in attr.cryptocoin.attributes.wallets {
            wallets.push(AssetWallet {
                asset_id: wallet.attributes.cryptocoin_id,
                asset_symbol: wallet.attributes.cryptocoin_symbol,
                balance: wallet.attributes.balance,
                class: AssetClass::Cryptocurrency,
                deleted: wallet.attributes.deleted,
                id: wallet.id,
                is_default: wallet.attributes.is_default,
                name: wallet.attributes.name,
            });
        }

        for wallet in attr.commodity.metal.attributes.wallets {
            wallets.push(AssetWallet {
                asset_id: wallet.attributes.cryptocoin_id,
                asset_symbol: wallet.attributes.cryptocoin_symbol,
                balance: wallet.attributes.balance,
                class: AssetClass::Metal,
                deleted: wallet.attributes.deleted,
                id: wallet.id,
                is_default: wallet.attributes.is_default,
                name: wallet.attributes.name,
            });
        }

        for wallet in attr.index.index.attributes.wallets {
            wallets.push(AssetWallet {
                asset_id: wallet.attributes.cryptocoin_id,
                asset_symbol: wallet.attributes.cryptocoin_symbol,
                balance: wallet.attributes.balance,
                class: AssetClass::Cryptoindex,
                deleted: wallet.attributes.deleted,
                id: wallet.id,
                is_default: wallet.attributes.is_default,
                name: wallet.attributes.name,
            });
        }

        for wallet in attr.security.etc.attributes.wallets {
            wallets.push(AssetWallet {
                asset_id: wallet.attributes.cryptocoin_id,
                asset_symbol: wallet.attributes.cryptocoin_symbol,
                balance: wallet.attributes.balance,
                class: AssetClass::Commodity,
                deleted: wallet.attributes.deleted,
                id: wallet.id,
                is_default: wallet.attributes.is_default,
                name: wallet.attributes.name,
            });
        }

        for wallet in attr.security.etf.attributes.wallets {
            wallets.push(AssetWallet {
                asset_id: wallet.attributes.cryptocoin_id,
                asset_symbol: wallet.attributes.cryptocoin_symbol,
                balance: wallet.attributes.balance,
                class: AssetClass::Etf,
                deleted: wallet.attributes.deleted,
                id: wallet.id,
                is_default: wallet.attributes.is_default,
                name: wallet.attributes.name,
            });
        }

        for wallet in attr.security.stock.attributes.wallets {
            wallets.push(AssetWallet {
                asset_id: wallet.attributes.cryptocoin_id,
                asset_symbol: wallet.attributes.cryptocoin_symbol,
                balance: wallet.attributes.balance,
                class: AssetClass::Stock,
                deleted: wallet.attributes.deleted,
                id: wallet.id,
                is_default: wallet.attributes.is_default,
                name: wallet.attributes.name,
            });
        }

        wallets
    }
}
