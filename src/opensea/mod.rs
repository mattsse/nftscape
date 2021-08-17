pub mod models;
pub mod query;

use crate::opensea::models::*;
use crate::ApiClient;

use crate::error::OpenSeaApiError;
use crate::opensea::query::*;
use serde::de::DeserializeOwned;

pub const API_BASE_MAINNET: &str = "https://api.opensea.io";
pub const API_BASE_RINKEBY: &str = "https://rinkeby-api.opensea.io";
pub const SITE_HOST_MAINNET: &str = "https://opensea.io";
pub const SITE_HOST_RINKEBY: &str = "https://rinkeby.opensea.io";

impl ApiClient {
    async fn request_json_opensea<T: DeserializeOwned>(
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<T> {
        Self::request_json::<_, OpenSeaApiError>(req).await
    }

    /// Get a list of orders from the orderbook, returning the page of orders
    /// and the count of total orders found.
    pub async fn get_orders(&self, query: &OrderQuery) -> anyhow::Result<OrderBook> {
        Self::request_json_opensea(
            self.client
                .get(self.join_url("wyvern/v1/orders")?)
                .query(query),
        )
        .await
    }

    /// Get a list of orders from the orderbook, returning the page of orders
    /// and the count of total orders found.
    pub async fn get_asset(
        &self,
        query: impl Into<OpenSeaAssetQuery>,
    ) -> anyhow::Result<Option<OpenSeaAsset>> {
        let OpenSeaAssetQuery {
            token_address,
            token_id,
        } = query.into();
        Self::request_json_opensea(self.client.get(self.join_url(format!(
            "api/v1/asset/{}/{}/",
            token_address,
            token_id.unwrap_or_default()
        ))?))
        .await
    }

    /// Fetch list of assets from the API, returning the page of assets and the
    /// count of total assets
    pub async fn get_assets(&self, query: &OpenSeaAssetsQuery) -> anyhow::Result<AssetList> {
        Self::request_json_opensea(
            self.client
                .get(self.join_url("api/v1/assets")?)
                .query(query),
        )
        .await
    }

    /// Fetch list of fungible tokens from the API matching parameters
    pub async fn get_payment_tokens(
        &self,
        query: &OpenSeaFungibleTokenQuery,
    ) -> anyhow::Result<FungibleTokenList> {
        Self::request_json_opensea(
            self.client
                .get(self.join_url("api/v1/tokens")?)
                .query(query),
        )
        .await
    }

    /// Fetch an bundle from the API
    /// Where slug is the bundle's identifier
    pub async fn get_bundle(
        &self,
        slug: impl AsRef<str>,
    ) -> anyhow::Result<Option<OpenSeaAssetBundle>> {
        Self::request_json_opensea(
            self.client
                .get(self.join_url(format!("api/v1/bundle/{}/", slug.as_ref(),))?),
        )
        .await
    }

    /// Fetch list of bundles from the API, returning the page of bundles and
    /// the count of total bundles
    pub async fn get_bundles(&self, query: &OpenSeaAssetBundleQuery) -> anyhow::Result<BundleList> {
        Self::request_json_opensea(
            self.client
                .get(self.join_url("api/v1/bundles")?)
                .query(query),
        )
        .await
    }
}
