# bitpanda-api

<p align="center">~ Rust client for Bitpanda API ~</p>
<p align="center">
  <a href="#get-started-">Get started</a>
  ·
  <a href="https://docs.rs/bitpanda-api" target="_blank">Documentation</a>
</p>

<p align="center">Developed by <a href="https://veeso.dev/" target="_blank">@veeso</a></p>
<p align="center">Current version: 0.1.0 (18/05/2023)</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"
    ><img
      src="https://img.shields.io/badge/License-MIT-teal.svg"
      alt="License-MIT"
  /></a>
  <a href="https://github.com/veeso/bitpanda-api-rs/stargazers"
    ><img
      src="https://img.shields.io/github/stars/veeso/bitpanda-api-rs.svg"
      alt="Repo stars"
  /></a>
  <a href="https://crates.io/crates/bitpanda-api"
    ><img
      src="https://img.shields.io/crates/d/bitpanda-api.svg"
      alt="Downloads counter"
  /></a>
  <a href="https://crates.io/crates/bitpanda-api"
    ><img
      src="https://img.shields.io/crates/v/bitpanda-api.svg"
      alt="Latest version"
  /></a>
  <a href="https://ko-fi.com/veeso">
    <img
      src="https://img.shields.io/badge/donate-ko--fi-red"
      alt="Ko-fi"
  /></a>
</p>
<p align="center">
  <a href="https://github.com/veeso/bitpanda-api-rs/actions"
    ><img
      src="https://github.com/veeso/bitpanda-api-rs/workflows/Build/badge.svg"
      alt="Build CI"
  /></a>
  <a href="https://coveralls.io/github/veeso/bitpanda-api-rs"
    ><img
      src="https://coveralls.io/repos/github/veeso/bitpanda-api-rs/badge.svg"
      alt="Coveralls"
  /></a>
  <a href="https://docs.rs/bitpanda-api"
    ><img
      src="https://docs.rs/bitpanda-api/badge.svg"
      alt="Docs"
  /></a>
</p>

---

- [bitpanda-api](#bitpanda-api)
  - [About bitpanda-api 🐼](#about-bitpanda-api-)
  - [Get started 🏁](#get-started-)
    - [Add bitpanda-api to your Cargo.toml 🦀](#add-bitpanda-api-to-your-cargotoml-)
    - [Query Bitpanda](#query-bitpanda)
  - [Documentation 📚](#documentation-)
  - [Support the developer ☕](#support-the-developer-)
  - [Contributing and issues 🤝🏻](#contributing-and-issues-)
  - [Changelog ⏳](#changelog-)
  - [License 📃](#license-)

---

## About bitpanda-api 🐼

bitpanda-api is a Rust client library for Bitpanda API.
The library supports both the "private API" which requires the APIKEY and the public API. These data are exposed:

- Public API
  - Get assets
  - Get OHLC for asset
- Private API
  - Get transactions
  - Get trades
  - Get wallets

---

## Get started 🏁

### Add bitpanda-api to your Cargo.toml 🦀

```toml
bitpanda-api = "^0.1.0"
```

### Query Bitpanda

```rust
use bitpanda_api::Client;
use bitpanda_api::model::AssetClass;
use bitpanda_api::model::ohlc::Period;

#[tokio::main]
async fn main() {

    let client = Client::default().x_apikey(env!("X_API_KEY"));

    // collect my last 20 trades
    client.get_trades_ex(Some(20)).await.expect("failed to collect trades");

    // get OHLC for BTC of the last 5 years
    let btc = client
        .get_assets(AssetClass::Cryptocurrency)
        .await
        .unwrap()
        .into_iter()
        .find(|asset| asset.symbol == "BTC")
        .unwrap();

    let ohlc = client.get_ohlc(Period::FiveYears, &btc.pid, "EUR").await.unwrap();
}
```

---

## Documentation 📚

The developer documentation can be found on Rust Docs at <https://docs.rs/bitpanda-api>

---

## Support the developer ☕

If you like bitpanda-api and you're grateful for the work I've done, please consider a little donation 🥳

You can make a donation with one of these platforms:

[![ko-fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/veeso)
[![PayPal](https://img.shields.io/badge/PayPal-00457C?style=for-the-badge&logo=paypal&logoColor=white)](https://www.paypal.me/chrisintin)
[![bitcoin](https://img.shields.io/badge/Bitcoin-ff9416?style=for-the-badge&logo=bitcoin&logoColor=white)](https://btc.com/bc1qvlmykjn7htz0vuprmjrlkwtv9m9pan6kylsr8w)

---

## Contributing and issues 🤝🏻

Contributions, bug reports, new features and questions are welcome! 😉
If you have any question or concern, or you want to suggest a new feature, or you want just want to improve bitpanda-api, feel free to open an issue or a PR.

Please follow [our contributing guidelines](CONTRIBUTING.md)

---

## Changelog ⏳

View bitpanda-api's changelog [HERE](CHANGELOG.md)

---

## License 📃

bitpanda-api is licensed under the MIT license.

You can read the entire license [HERE](LICENSE)
