use rust_decimal::Decimal;

use crate::model::FiatWallet;

#[derive(Deserialize)]
pub struct FiatWalletResponse {
    data: Vec<Wallet>,
}

#[derive(Deserialize)]
pub struct Wallet {
    id: String,
    attributes: Attributes,
}

#[derive(Deserialize)]
pub struct Attributes {
    fiat_id: String,
    fiat_symbol: String,
    balance: Decimal,
    name: String,
    pending_transactions_count: usize,
}

impl FiatWalletResponse {
    pub fn into_fiat_wallets(self) -> Vec<FiatWallet> {
        self.data
            .into_iter()
            .map(|wallet| FiatWallet {
                balance: wallet.attributes.balance,
                fiat_id: wallet.attributes.fiat_id,
                id: wallet.id,
                name: wallet.attributes.name,
                pending_transactions_count: wallet.attributes.pending_transactions_count,
                symbol: wallet.attributes.fiat_symbol,
            })
            .collect()
    }
}
