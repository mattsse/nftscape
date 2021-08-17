pub mod models;

use crate::rarible::models::*;
use crate::ApiClient;

use crate::error::RaribleApiError;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

impl ApiClient {
    async fn request_json_rarible<T: DeserializeOwned>(
        req: reqwest::RequestBuilder,
    ) -> anyhow::Result<T> {
        Self::request_json::<_, RaribleApiError>(req).await
    }

    async fn post_json_rarible<T, U, B>(&self, url: U, body: Option<&B>) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
        U: IntoUrl,
        B: Serialize + ?Sized,
    {
        self.post_json::<_, _, _, RaribleApiError>(url, body).await
    }

    pub async fn get_currency_rate(
        &self,
        blockchain: &str,
        address: &str,
        at: u64,
    ) -> anyhow::Result<CurrencyRate> {
        let request = self
            .client
            .get(self.join_url("protocol/v0.1/ethereum/currency/rate")?);

        Self::request_json_rarible(request.query(&[
            ("blockchain", &blockchain.to_string()),
            ("address", &address.to_string()),
            ("at", &at.to_string()),
        ]))
        .await
    }

    pub async fn get_erc20_balance(
        &self,
        contract: &str,
        owner: &str,
    ) -> anyhow::Result<Erc20DecimalBalance> {
        let request = self.client.get(self.join_url(format!(
            "protocol/v0.1/ethereum/erc20/balances/{}/{}",
            urlencode(contract),
            urlencode(owner)
        ))?);
        Self::request_json_rarible(request).await
    }

    pub async fn get_erc20_token_by_id(&self, contract: &str) -> anyhow::Result<Erc20Token> {
        let request = self.client.get(self.join_url(format!(
            "protocol/v0.1/ethereum/erc20/tokens/{}",
            urlencode(contract),
        ))?);
        Self::request_json_rarible(request).await
    }

    pub async fn create_gateway_pending_transactions(
        &self,
        create_transaction: CreateTransactionRequest,
    ) -> anyhow::Result<Vec<LogEvent>> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/transactions")?,
            Some(&create_transaction),
        )
        .await
    }

    pub async fn create_lock(&self, item_id: &str, lock_form: LockForm) -> anyhow::Result<Lock> {
        self.post_json_rarible(
            self.join_url(format!(
                "protocol/v0.1/ethereum/unlockable/item/{}/lock",
                urlencode(item_id)
            ))?,
            Some(&lock_form),
        )
        .await
    }

    pub async fn get_nft_activities(
        &self,
        nft_activity_filter: NftActivityFilter,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<NftActivities> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/activities/search")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }

        self.post_json_rarible(url, Some(&nft_activity_filter))
            .await
    }

    /// Returns next available tokenId for minter
    pub async fn generate_nft_token_id(
        &self,
        collection: &str,
        minter: &str,
    ) -> anyhow::Result<NftTokenId> {
        let request = self.client.get(self.join_url(format!(
            "protocol/v0.1/ethereum/nft/collections/{}/generate_token_id",
            urlencode(collection)
        ))?);

        Self::request_json_rarible(request.query(&[("minter", minter)])).await
    }

    /// Returns Collection by address
    pub async fn get_nft_collection_by_id(
        &self,
        collection: &str,
    ) -> anyhow::Result<NftCollection> {
        let request = self.client.get(self.join_url(format!(
            "protocol/v0.1/ethereum/nft/collections/{}",
            urlencode(collection)
        ))?);

        Self::request_json_rarible(request).await
    }

    pub async fn search_nft_all_collections(
        &self,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<NftCollections> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/collections/all")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn search_nft_collections_by_owner(
        &self,
        owner: &str,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<NftCollections> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/collections/byOwner")?;
        url.query_pairs_mut().append_pair("owner", owner);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_all_items(
        &self,
        continuation: Option<&str>,
        size: Option<i32>,
        show_deleted: Option<bool>,
        last_updated_from: Option<i64>,
        last_updated_to: Option<i64>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<NftItems> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/items/all")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(show_deleted) = show_deleted {
            url.query_pairs_mut()
                .append_pair("showDeleted", &show_deleted.to_string());
        }
        if let Some(last_updated_from) = last_updated_from {
            url.query_pairs_mut()
                .append_pair("lastUpdatedFrom", &last_updated_from.to_string());
        }
        if let Some(last_updated_to) = last_updated_to {
            url.query_pairs_mut()
                .append_pair("lastUpdatedTo", &last_updated_to.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// returns Item by identifier
    pub async fn get_nft_item_by_id(
        &self,
        item_id: &str,
        include_meta: Option<bool>,
    ) -> anyhow::Result<NftItem> {
        let mut url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft/items/{}",
            urlencode(item_id)
        ))?;
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_item_meta_by_id(&self, item_id: &str) -> anyhow::Result<NftItemMeta> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft/items/{}/meta",
            urlencode(item_id)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_items_by_collection(
        &self,
        collection: &str,
        continuation: Option<&str>,
        size: Option<i32>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<NftItems> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/items/byCollection")?;

        url.query_pairs_mut().append_pair("collection", collection);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_items_by_creator(
        &self,
        creator: &str,
        continuation: Option<&str>,
        size: Option<i32>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<NftItems> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/items/byCreator")?;

        url.query_pairs_mut().append_pair("creator", creator);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }
    pub async fn get_nft_items_by_owner(
        &self,
        owner: &str,
        continuation: Option<&str>,
        size: Option<i32>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<NftItems> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/items/byOwner")?;

        url.query_pairs_mut().append_pair("owner", owner);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_lazy_item_by_id(&self, item_id: &str) -> anyhow::Result<LazyNft> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft/items/{}/lazy",
            urlencode(item_id)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Lazy mint of NFT
    pub async fn mint_nft_asset(&self, lazy_nft: LazyNft) -> anyhow::Result<NftItem> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/nft/mints")?,
            Some(&lazy_nft),
        )
        .await
    }

    pub async fn get_nft_order_activities_by_collection(
        &self,
        _type: Vec<String>,
        collection: &str,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<Activities> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/activities/byCollection")?;

        url.query_pairs_mut()
            .append_pair("type", &_type.join(","))
            .append_pair("collection", collection);

        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_activities_by_item(
        &self,
        _type: Vec<String>,
        contract: &str,
        token_id: &str,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<Activities> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/activities/byItem")?;
        url.query_pairs_mut()
            .append_pair("type", &_type.join(","))
            .append_pair("contract", contract)
            .append_pair("token_id", token_id);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_activities_by_user(
        &self,
        _type: Vec<String>,
        user: Vec<String>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<Activities> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/activities/byUser")?;

        url.query_pairs_mut()
            .append_pair("type", &_type.join(","))
            .append_pair("user", &user.join(","));
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_all_activities(
        &self,
        _type: Vec<String>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<Activities> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/activities/all")?;
        url.query_pairs_mut().append_pair("type", &_type.join(","));
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Returns next available tokenId for minter
    pub async fn generate_nft_order_token_id(
        &self,
        collection: &str,
        minter: &str,
    ) -> anyhow::Result<NftTokenId> {
        let mut url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft-order/collections/{}/generate_token_id",
            urlencode(collection)
        ))?;
        url.query_pairs_mut().append_pair("minter", minter);
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Returns Collection by address
    pub async fn get_nft_order_collection_by_id(
        &self,
        collection: &str,
    ) -> anyhow::Result<NftCollection> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft-order/collections/{}",
            urlencode(collection)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn search_nft_order_all_collections(
        &self,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<NftCollections> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/collections/all")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn search_nft_order_collections_by_owner(
        &self,
        owner: &str,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<NftCollections> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/collections/byOwner")?;
        url.query_pairs_mut().append_pair("owner", owner);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_all_items(
        &self,
        continuation: Option<&str>,
        size: Option<i32>,
        show_deleted: Option<bool>,
        last_updated_from: Option<i64>,
        last_updated_to: Option<i64>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<PageNftOrderItem> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/items/all")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(show_deleted) = show_deleted {
            url.query_pairs_mut()
                .append_pair("showDeleted", &show_deleted.to_string());
        }
        if let Some(last_updated_from) = last_updated_from {
            url.query_pairs_mut()
                .append_pair("lastUpdatedFrom", &last_updated_from.to_string());
        }
        if let Some(last_updated_to) = last_updated_to {
            url.query_pairs_mut()
                .append_pair("lastUpdatedTo", &last_updated_to.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_item_by_id(
        &self,
        item_id: &str,
        include_meta: Option<bool>,
    ) -> anyhow::Result<NftOrderItem> {
        let mut url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft-order/items/{}",
            urlencode(item_id)
        ))?;
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_item_meta_by_id(
        &self,
        item_id: &str,
    ) -> anyhow::Result<NftItemMeta> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft-order/items/{}/meta",
            urlencode(item_id)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_items_by_collection(
        &self,
        collection: &str,
        continuation: Option<&str>,
        size: Option<i32>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<PageNftOrderItem> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/items/byCollection")?;

        url.query_pairs_mut().append_pair("collection", collection);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_items_by_creator(
        &self,
        creator: &str,
        continuation: Option<&str>,
        size: Option<i32>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<PageNftOrderItem> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/items/byCreator")?;

        url.query_pairs_mut().append_pair("creator", creator);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_items_by_owner(
        &self,
        owner: &str,
        continuation: Option<&str>,
        size: Option<i32>,
        include_meta: Option<bool>,
    ) -> anyhow::Result<PageNftOrderItem> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/items/byOwner")?;

        url.query_pairs_mut().append_pair("owner", owner);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(include_meta) = include_meta {
            url.query_pairs_mut()
                .append_pair("includeMeta", &include_meta.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_lazy_item_by_id(&self, item_id: &str) -> anyhow::Result<LazyNft> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft-order/items/{}/lazy",
            urlencode(item_id)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Lazy mint of NFT
    pub async fn mint_nft_order_asset(&self, lazy_nft: LazyNft) -> anyhow::Result<NftOrderItem> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/nft-order/mints")?,
            Some(&lazy_nft),
        )
        .await
    }

    pub async fn get_nft_order_all_ownerships(
        &self,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<PageNftOrderOwnershipItem> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/ownerships/all")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_ownership_by_id(
        &self,
        ownership_id: &str,
    ) -> anyhow::Result<NftOrderOwnership> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft-order/ownerships/{}",
            urlencode(ownership_id)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_order_ownerships_by_item(
        &self,
        contract: &str,
        token_id: &str,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<PageNftOrderOwnershipItem> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft-order/activities/byItem")?;
        url.query_pairs_mut()
            .append_pair("contract", contract)
            .append_pair("token_id", token_id);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_all_ownerships(
        &self,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<NftOwnerships> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/ownerships/all")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_ownership_by_id(
        &self,
        ownership_id: &str,
    ) -> anyhow::Result<NftOwnership> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/nft/ownerships/{}",
            urlencode(ownership_id)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_nft_ownerships_by_item(
        &self,
        contract: &str,
        token_id: &str,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<NftOwnerships> {
        let mut url = self.join_url("protocol/v0.1/ethereum/nft/ownerships/byItem")?;
        url.query_pairs_mut()
            .append_pair("contract", contract)
            .append_pair("token_id", token_id);
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Create pending transaction for NFT
    pub async fn create_nft_pending_transaction(
        &self,
        create_transaction: CreateTransactionRequest,
    ) -> anyhow::Result<Vec<LogEvent>> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/nft/transactions")?,
            Some(&create_transaction),
        )
        .await
    }

    pub async fn get_order_activities(
        &self,
        order_activity_filter: OrderActivityFilter,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrderActivities> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/activities/search")?;
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        self.post_json_rarible(url, Some(&order_activity_filter))
            .await
    }

    /// Aggregate nft purchase by collection
    pub async fn aggregate_nft_purchase_buy_collection(
        &self,
        start_date: i64,
        end_date: i64,
        size: Option<i64>,
        source: Option<AggregationSource>,
    ) -> anyhow::Result<Vec<AggregationData>> {
        let mut url =
            self.join_url("protocol/v0.1/ethereum/order/aggregations/nftPurchaseByCollection")?;

        url.query_pairs_mut()
            .append_pair("startDate", &start_date.to_string())
            .append_pair("endDate", &end_date.to_string());

        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(source) = source {
            url.query_pairs_mut()
                .append_pair("source", &source.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Aggregate nft purchase by taker
    pub async fn aggregate_nft_purchase_by_taker(
        &self,
        start_date: i64,
        end_date: i64,
        size: Option<i64>,
        source: Option<AggregationSource>,
    ) -> anyhow::Result<Vec<AggregationData>> {
        let mut url =
            self.join_url("protocol/v0.1/ethereum/order/aggregations/nftPurchaseByTaker")?;

        url.query_pairs_mut()
            .append_pair("startDate", &start_date.to_string())
            .append_pair("endDate", &end_date.to_string());

        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(source) = source {
            url.query_pairs_mut()
                .append_pair("source", &source.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Aggregate nft sell order by maker
    pub async fn aggregate_nft_sell_by_maker(
        &self,
        start_date: i64,
        end_date: i64,
        size: Option<i64>,
        source: Option<AggregationSource>,
    ) -> anyhow::Result<Vec<AggregationData>> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/aggregations/nftSellByMaker")?;
        url.query_pairs_mut()
            .append_pair("startDate", &start_date.to_string())
            .append_pair("endDate", &end_date.to_string());

        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        if let Some(source) = source {
            url.query_pairs_mut()
                .append_pair("source", &source.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    #[warn(clippy::too_many_arguments)]
    pub async fn get_bids_by_item(
        &self,
        contract: &str,
        token_id: &str,
        status: Vec<OrderBidStatus>,
        maker: Option<&str>,
        start_date: Option<String>,
        end_date: Option<String>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrderBidsPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/bids/byItem")?;
        url.query_pairs_mut()
            .append_pair("contract", contract)
            .append_pair("token_id", token_id)
            .append_pair(
                "status",
                &status
                    .iter()
                    .map(OrderBidStatus::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            );

        if let Some(maker) = maker {
            url.query_pairs_mut().append_pair("maker", maker);
        }
        if let Some(start_date) = start_date {
            url.query_pairs_mut().append_pair("startDate", &start_date);
        }
        if let Some(end_date) = end_date {
            url.query_pairs_mut().append_pair("endDate", &end_date);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_order_bids_by_item(
        &self,
        contract: &str,
        token_id: &str,
        maker: Option<&str>,
        origin: Option<&str>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrdersPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/orders/bids/byItem")?;
        url.query_pairs_mut()
            .append_pair("contract", contract)
            .append_pair("token_id", token_id);

        if let Some(maker) = maker {
            url.query_pairs_mut().append_pair("maker", maker);
        }
        if let Some(origin) = origin {
            url.query_pairs_mut().append_pair("origin", origin);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_order_bids_by_maker(
        &self,
        maker: &str,
        origin: Option<&str>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrdersPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/orders/bids/byMaker")?;
        url.query_pairs_mut().append_pair("maker", maker);

        if let Some(origin) = origin {
            url.query_pairs_mut().append_pair("origin", origin);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_order_by_hash(&self, hash: &str) -> anyhow::Result<Order> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/order/orders/{}",
            urlencode(hash)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_orders_all(
        &self,
        origin: Option<&str>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrdersPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/orders/all")?;

        if let Some(origin) = origin {
            url.query_pairs_mut().append_pair("origin", origin);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_sell_orders(
        &self,
        origin: Option<&str>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrdersPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/orders/sell")?;
        if let Some(origin) = origin {
            url.query_pairs_mut().append_pair("origin", origin);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_sell_orders_by_collection(
        &self,
        collection: &str,
        origin: Option<&str>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrdersPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/orders/sell/byCollection")?;
        url.query_pairs_mut().append_pair("collection", collection);
        if let Some(origin) = origin {
            url.query_pairs_mut().append_pair("origin", origin);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_sell_orders_by_item(
        &self,
        contract: &str,
        token_id: &str,
        maker: Option<&str>,
        origin: Option<&str>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrdersPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/orders/sell/byItem")?;
        url.query_pairs_mut()
            .append_pair("contract", contract)
            .append_pair("token_id", token_id);

        if let Some(maker) = maker {
            url.query_pairs_mut().append_pair("maker", maker);
        }
        if let Some(origin) = origin {
            url.query_pairs_mut().append_pair("origin", origin);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn get_sell_orders_by_maker(
        &self,
        maker: &str,
        origin: Option<&str>,
        continuation: Option<&str>,
        size: Option<i32>,
    ) -> anyhow::Result<OrdersPagination> {
        let mut url = self.join_url("protocol/v0.1/ethereum/order/orders/sell/byMaker")?;
        url.query_pairs_mut().append_pair("maker", maker);

        if let Some(origin) = origin {
            url.query_pairs_mut().append_pair("origin", origin);
        }
        if let Some(continuation) = continuation {
            url.query_pairs_mut()
                .append_pair("continuation", continuation);
        }
        if let Some(size) = size {
            url.query_pairs_mut().append_pair("size", &size.to_string());
        }
        Self::request_json_rarible(self.client.get(url)).await
    }

    pub async fn invert_order(
        &self,
        hash: &str,
        invert_order_form: InvertOrderForm,
    ) -> anyhow::Result<OrderForm> {
        self.post_json_rarible(
            self.join_url(format!(
                "protocol/v0.1/ethereum/order/orders/{}/invert",
                urlencode(hash)
            ))?,
            Some(&invert_order_form),
        )
        .await
    }

    pub async fn prepare_order_cancel_transaction(
        &self,
        hash: &str,
    ) -> anyhow::Result<PreparedOrderTx> {
        self.post_json_rarible(
            self.join_url(format!(
                "protocol/v0.1/ethereum/order/orders/{}/prepareCancelTx",
                urlencode(hash)
            ))?,
            Some(&()),
        )
        .await
    }

    pub async fn prepare_order_transaction(
        &self,
        hash: &str,
        prepare_order_tx_form: PrepareOrderTxForm,
    ) -> anyhow::Result<PrepareOrderTxResponse> {
        self.post_json_rarible(
            self.join_url(format!(
                "protocol/v0.1/ethereum/order/orders/{}/prepareTx",
                urlencode(hash)
            ))?,
            Some(&prepare_order_tx_form),
        )
        .await
    }

    pub async fn prepare_order_v2_transaction(
        &self,
        hash: &str,
        order_form: OrderForm,
    ) -> anyhow::Result<PrepareOrderTxResponse> {
        self.post_json_rarible(
            self.join_url(format!(
                "protocol/v0.1/ethereum/order/orders/{}/prepareV2Tx",
                urlencode(hash)
            ))?,
            Some(&order_form),
        )
        .await
    }

    pub async fn update_order_make_stock(&self, hash: &str) -> anyhow::Result<Order> {
        let url = self.join_url(format!(
            "protocol/v0.1/ethereum/order/orders/{}/updateMakeStock",
            urlencode(hash)
        ))?;
        Self::request_json_rarible(self.client.get(url)).await
    }

    /// Create or update Order
    pub async fn upsert_order(&self, order_form: OrderForm) -> anyhow::Result<Order> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/order/orders")?,
            Some(&order_form),
        )
        .await
    }

    pub async fn encode_order(&self, order_form: OrderForm) -> anyhow::Result<EncodedOrder> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/order/encoder/order")?,
            Some(&order_form),
        )
        .await
    }

    pub async fn encode_order_asset_type(
        &self,
        asset_type: AssetType,
    ) -> anyhow::Result<EncodedOrderData> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/order/encoder/assetType")?,
            Some(&asset_type),
        )
        .await
    }

    pub async fn encode_order_data(
        &self,
        order_data: OrderData,
    ) -> anyhow::Result<EncodedOrderData> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/order/encoder/data")?,
            Some(&order_data),
        )
        .await
    }

    /// Create pending transaction for order
    pub async fn create_order_pending_transaction(
        &self,
        create_transaction_request: CreateTransactionRequest,
    ) -> anyhow::Result<Vec<LogEvent>> {
        self.post_json_rarible(
            self.join_url("protocol/v0.1/ethereum/order/transactions")?,
            Some(&create_transaction_request),
        )
        .await
    }
}
