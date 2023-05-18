//! # Bitpanda API client

use async_recursion::async_recursion;

use super::{ApiError, ApiResult};
use crate::model::crypto_wallet::CryptoWalletTransaction;
use crate::model::fiat_wallet::FiatWalletTransaction;
use crate::model::ohlc::Period;
use crate::model::{
    Asset, AssetClass, AssetWallet, CryptoWallet, FiatWallet, OpenHighLowCloseChart, Trade,
    TransactionStatus, TransactionType,
};

mod asset_wallet_response;
mod crypto_wallet_response;
mod crypto_wallet_tx_response;
mod fiat_wallet_response;
mod fiat_wallet_tx_response;
mod get_assets_response;
mod get_ohlc_response;
mod trade_response;

use asset_wallet_response::AssetWalletResponse;
use crypto_wallet_response::CryptoWalletResponse;
use crypto_wallet_tx_response::CryptoWalletTxResponse;
use fiat_wallet_response::FiatWalletResponse;
use fiat_wallet_tx_response::FiatWalletTxResponse;
use get_assets_response::GetAssetsResponse;
use get_ohlc_response::GetOhlcResponse;
use trade_response::TradeResponse;

const BITPANDA_API_URL: &str = "https://api.bitpanda.com/v1";
const BITPANDA_PUBLIC_URL: &str = "https://api.bitpanda.com";
const TRADE_DEFAULT_PAGE_SIZE: usize = 25;
const ASSETS_DEFAULT_PAGE_SIZE: usize = 500;

/// Bitpanda api client
#[derive(Default)]
pub struct Client {
    x_apikey: Option<String>,
}

impl Client {
    /// Construct client with x-apikey
    pub fn x_apikey(mut self, apikey: impl ToString) -> Self {
        self.x_apikey = Some(apikey.to_string());

        self
    }

    // requests

    /// Get asset wallets for user.
    /// Requires APIKEY
    pub async fn get_asset_wallets(&self) -> ApiResult<Vec<AssetWallet>> {
        let response: AssetWalletResponse = self
            .request_with_auth("asset-wallets")?
            .send()
            .await?
            .json()
            .await?;

        Ok(response.into_asset_wallets())
    }

    /// Get crypto wallets
    /// Requires APIKEY
    pub async fn get_crypto_wallets(&self) -> ApiResult<Vec<CryptoWallet>> {
        let response: CryptoWalletResponse = self
            .request_with_auth("wallets")?
            .send()
            .await?
            .json()
            .await?;

        Ok(response.into_crypto_wallets())
    }

    /// Get FIAT wallets
    /// Requires APIKEY
    pub async fn get_fiat_wallets(&self) -> ApiResult<Vec<FiatWallet>> {
        let response: FiatWalletResponse = self
            .request_with_auth("fiatwallets")?
            .send()
            .await?
            .json()
            .await?;

        Ok(response.into_fiat_wallets())
    }

    /// get user's trades.
    /// Requires APIKEY
    pub async fn get_trades(&self) -> ApiResult<Vec<Trade>> {
        self.get_trades_ex(None).await
    }

    /// get user's trades.
    /// If max_results is specified, only the amount of trades specified are fetched
    /// Requires APIKEY
    pub async fn get_trades_ex(&self, max_results: Option<usize>) -> ApiResult<Vec<Trade>> {
        self.do_get_trades(vec![], 0, max_results).await
    }

    /// Get crypto wallet transactions
    /// Requires APIKEY
    pub async fn get_crypto_wallet_transactions(&self) -> ApiResult<Vec<CryptoWalletTransaction>> {
        self.get_crypto_wallet_transactions_ex(None, None, None)
            .await
    }

    /// Get crypto wallet transactions
    /// if specified, get transactions with provided filters
    /// Requires APIKEY
    pub async fn get_crypto_wallet_transactions_ex(
        &self,
        transaction_type: Option<TransactionType>,
        status: Option<TransactionStatus>,
        max_results: Option<usize>,
    ) -> ApiResult<Vec<CryptoWalletTransaction>> {
        self.do_get_crypto_wallet_transactions(vec![], transaction_type, status, 0, max_results)
            .await
    }

    /// Get fiat wallet transactions
    /// Requires APIKEY
    pub async fn get_fiat_wallet_transactions(&self) -> ApiResult<Vec<FiatWalletTransaction>> {
        self.get_fiat_wallet_transactions_ex(None, None, None).await
    }

    /// Get fiat wallet transactions
    /// if specified, get transactions with provided filters
    /// Requires APIKEY
    pub async fn get_fiat_wallet_transactions_ex(
        &self,
        transaction_type: Option<TransactionType>,
        status: Option<TransactionStatus>,
        max_results: Option<usize>,
    ) -> ApiResult<Vec<FiatWalletTransaction>> {
        self.do_get_fiat_wallet_transactions(vec![], transaction_type, status, 0, max_results)
            .await
    }

    /// Get assets available on Bitpanda by class
    pub async fn get_assets(&self, asset_class: AssetClass) -> ApiResult<Vec<Asset>> {
        self.do_get_assets(vec![], asset_class, 0).await
    }

    /// get OHLC for provided symbols
    pub async fn get_ohlc(
        &self,
        period: Period,
        pid: &str,
        currency: &str,
    ) -> ApiResult<OpenHighLowCloseChart> {
        let url = format!("ohlc/{pid}/{currency}/{}", period.to_string());

        Ok(self
            .pub_request_v3(url)
            .send()
            .await?
            .json::<GetOhlcResponse>()
            .await?
            .into_ohlc(period))
    }

    #[async_recursion]
    async fn do_get_trades(
        &self,
        mut trades: Vec<Trade>,
        page: usize,
        max_results: Option<usize>,
    ) -> ApiResult<Vec<Trade>> {
        let page_size = match max_results {
            Some(sz) if trades.len() + TRADE_DEFAULT_PAGE_SIZE > sz => {
                sz.checked_sub(trades.len()).unwrap_or_default()
            }
            Some(_) | None => TRADE_DEFAULT_PAGE_SIZE,
        };
        let url = format!("trades?page={page}&page_size={page_size}");
        trace!("next get trade url: {url}");

        let response: TradeResponse = self.request_with_auth(url)?.send().await?.json().await?;

        let next_page = response.next_page();

        trades.extend(response.into_trades()?);

        if let Some(max_results) = max_results {
            if trades.len() >= max_results {
                return Ok(trades);
            }
        }

        if let Some(page) = next_page {
            trace!("there are still trades to be fetched");
            self.do_get_trades(trades, page, max_results).await
        } else {
            Ok(trades)
        }
    }

    #[async_recursion]
    async fn do_get_crypto_wallet_transactions(
        &self,
        mut txs: Vec<CryptoWalletTransaction>,
        transaction_type: Option<TransactionType>,
        status: Option<TransactionStatus>,
        page: usize,
        max_results: Option<usize>,
    ) -> ApiResult<Vec<CryptoWalletTransaction>> {
        let page_size = match max_results {
            Some(sz) if txs.len() + TRADE_DEFAULT_PAGE_SIZE > sz => {
                sz.checked_sub(txs.len()).unwrap_or_default()
            }
            Some(_) | None => TRADE_DEFAULT_PAGE_SIZE,
        };

        let transaction_type_arg = transaction_type
            .map(|t| format!("&type={}", t.to_string()))
            .unwrap_or_default();

        let status_arg = status
            .map(|s| format!("&status={}", s.to_string()))
            .unwrap_or_default();

        let url = format!("wallets/transactions?page={page}&page_size={page_size}{transaction_type_arg}{status_arg}");
        trace!("next get crypto transactions url: {url}");

        let response: CryptoWalletTxResponse =
            self.request_with_auth(url)?.send().await?.json().await?;

        let next_page = response.next_page();

        txs.extend(response.into_transactions()?);

        if let Some(max_results) = max_results {
            if txs.len() >= max_results {
                return Ok(txs);
            }
        }

        if let Some(page) = next_page {
            trace!("there are still tx to be fetched");
            self.do_get_crypto_wallet_transactions(txs, transaction_type, status, page, max_results)
                .await
        } else {
            Ok(txs)
        }
    }

    #[async_recursion]
    async fn do_get_fiat_wallet_transactions(
        &self,
        mut txs: Vec<FiatWalletTransaction>,
        transaction_type: Option<TransactionType>,
        status: Option<TransactionStatus>,
        page: usize,
        max_results: Option<usize>,
    ) -> ApiResult<Vec<FiatWalletTransaction>> {
        let page_size = match max_results {
            Some(sz) if txs.len() + TRADE_DEFAULT_PAGE_SIZE > sz => {
                sz.checked_sub(txs.len()).unwrap_or_default()
            }
            Some(_) | None => TRADE_DEFAULT_PAGE_SIZE,
        };

        let transaction_type_arg = transaction_type
            .map(|t| format!("&type={}", t.to_string()))
            .unwrap_or_default();

        let status_arg = status
            .map(|s| format!("&status={}", s.to_string()))
            .unwrap_or_default();

        let url = format!("fiatwallets/transactions?page={page}&page_size={page_size}{transaction_type_arg}{status_arg}");
        trace!("next get crypto transactions url: {url}");

        let response: FiatWalletTxResponse =
            self.request_with_auth(url)?.send().await?.json().await?;
        let next_page = response.next_page();

        txs.extend(response.into_transactions()?);

        if let Some(max_results) = max_results {
            if txs.len() >= max_results {
                return Ok(txs);
            }
        }

        if let Some(page) = next_page {
            trace!("there are still tx to be fetched");
            self.do_get_fiat_wallet_transactions(txs, transaction_type, status, page, max_results)
                .await
        } else {
            Ok(txs)
        }
    }

    #[async_recursion]
    async fn do_get_assets(
        &self,
        mut assets: Vec<Asset>,
        asset_class: AssetClass,
        page: usize,
    ) -> ApiResult<Vec<Asset>> {
        let url = format!(
            "assets?page={page}&page_size={ASSETS_DEFAULT_PAGE_SIZE}&type[]={}",
            asset_class.to_string()
        );
        trace!("next get assets url: {url}");

        let response: GetAssetsResponse = self.pub_request_v3(url).send().await?.json().await?;

        let next_page = response.next_page();

        assets.extend(response.into_assets(asset_class));

        if let Some(page) = next_page {
            trace!("there are still assets to be fetched");
            self.do_get_assets(assets, asset_class, page).await
        } else {
            Ok(assets)
        }
    }

    fn priv_request(&self, url: impl ToString) -> reqwest::RequestBuilder {
        reqwest::Client::new().get(format!("{BITPANDA_API_URL}/{}", url.to_string()))
    }

    fn pub_request_v3(&self, url: impl ToString) -> reqwest::RequestBuilder {
        reqwest::Client::new().get(format!("{BITPANDA_PUBLIC_URL}/v3/{}", url.to_string()))
    }

    fn request_with_auth(&self, url: impl ToString) -> ApiResult<reqwest::RequestBuilder> {
        if let Some(apikey) = &self.x_apikey {
            Ok(self.priv_request(url).header("X-API-KEY", apikey))
        } else {
            Err(ApiError::Unauthorized)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn should_get_asset_wallets() {
        assert!(client().get_asset_wallets().await.is_ok());
    }

    #[tokio::test]
    async fn should_get_crypto_wallets() {
        assert!(client().get_crypto_wallets().await.is_ok());
    }

    #[tokio::test]
    async fn should_get_fiat_wallets() {
        assert!(client().get_fiat_wallets().await.is_ok());
    }

    #[tokio::test]
    async fn should_get_all_trades() {
        assert!(client().get_trades().await.is_ok());
    }

    #[tokio::test]
    async fn should_get_limited_trades() {
        assert_eq!(client().get_trades_ex(Some(128)).await.unwrap().len(), 128);
    }

    #[tokio::test]
    async fn should_get_crypto_transactions() {
        assert!(client().get_crypto_wallet_transactions().await.is_ok());
    }

    #[tokio::test]
    async fn should_get_crypto_transactions_limited() {
        assert_eq!(
            client()
                .get_crypto_wallet_transactions_ex(None, None, Some(45))
                .await
                .unwrap()
                .len(),
            45
        );
    }

    #[tokio::test]
    async fn should_get_crypto_transactions_by_type() {
        assert!(client()
            .get_crypto_wallet_transactions_ex(Some(TransactionType::Buy), None, Some(25))
            .await
            .unwrap()
            .iter()
            .all(|t| t.transaction_type == TransactionType::Buy));
    }

    #[tokio::test]
    async fn should_get_crypto_transactions_by_status() {
        assert!(client()
            .get_crypto_wallet_transactions_ex(None, Some(TransactionStatus::Canceled), Some(25))
            .await
            .unwrap()
            .iter()
            .all(|t| t.status == TransactionStatus::Canceled));
    }

    #[tokio::test]
    async fn should_get_fiat_transactions_limited() {
        assert_eq!(
            client()
                .get_fiat_wallet_transactions_ex(None, None, Some(45))
                .await
                .unwrap()
                .len(),
            45
        );
    }

    #[tokio::test]
    async fn should_get_fiat_transactions_by_type() {
        assert!(client()
            .get_fiat_wallet_transactions_ex(Some(TransactionType::Buy), None, Some(25))
            .await
            .unwrap()
            .iter()
            .all(|t| t.transaction_type == TransactionType::Buy));
    }

    #[tokio::test]
    async fn should_get_fiat_transactions_by_status() {
        assert!(client()
            .get_fiat_wallet_transactions_ex(None, Some(TransactionStatus::Canceled), Some(25))
            .await
            .unwrap()
            .iter()
            .all(|t| t.status == TransactionStatus::Canceled));
    }

    #[tokio::test]
    async fn should_get_assets() {
        assert!(client()
            .get_assets(AssetClass::Cryptocurrency)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn should_get_ohlc_for_btc() {
        let client = client();

        let btc = client
            .get_assets(AssetClass::Cryptocurrency)
            .await
            .unwrap()
            .into_iter()
            .find(|asset| asset.symbol == "BTC")
            .unwrap();

        assert!(client.get_ohlc(Period::Day, &btc.pid, "EUR").await.is_ok());
        assert!(client.get_ohlc(Period::Week, &btc.pid, "EUR").await.is_ok());
        assert!(client
            .get_ohlc(Period::Month, &btc.pid, "EUR")
            .await
            .is_ok());
        assert!(client.get_ohlc(Period::Year, &btc.pid, "EUR").await.is_ok());
        assert!(client
            .get_ohlc(Period::FiveYears, &btc.pid, "EUR")
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn should_return_error_if_unauthorized() {
        let client = Client::default();
        assert!(client.get_asset_wallets().await.is_err());
    }

    fn client() -> Client {
        log_init();
        Client::default().x_apikey(env!("X_API_KEY"))
    }

    fn log_init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
}
