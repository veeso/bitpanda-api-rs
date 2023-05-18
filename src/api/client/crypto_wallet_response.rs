use rust_decimal::Decimal;

use crate::model::CryptoWallet;

#[derive(Deserialize)]
pub struct CryptoWalletResponse {
    data: Vec<Wallet>,
}

#[derive(Deserialize)]
pub struct Wallet {
    attributes: Attributes,
    id: String,
}

#[derive(Deserialize)]
pub struct Attributes {
    balance: Decimal,
    cryptocoin_id: String,
    cryptocoin_symbol: String,
    deleted: bool,
    is_default: bool,
    name: String,
    pending_transactions_count: usize,
}

impl CryptoWalletResponse {
    pub fn into_crypto_wallets(self) -> Vec<CryptoWallet> {
        self.data
            .into_iter()
            .map(|wallet| CryptoWallet {
                id: wallet.id,
                balance: wallet.attributes.balance,
                cryptocoin_id: wallet.attributes.cryptocoin_id,
                deleted: wallet.attributes.deleted,
                is_default: wallet.attributes.is_default,
                name: wallet.attributes.name,
                pending_transactions_count: wallet.attributes.pending_transactions_count,
                symbol: wallet.attributes.cryptocoin_symbol,
            })
            .collect()
    }
}
