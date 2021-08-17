use crate::opensea::models::{AuctionType, SaleKind};
use serde::Serialize;

//extends Partial<OrderJSON>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OrderQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_kind: Option<SaleKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_english: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_expired: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_invalid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listed_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl OrderQuery {
    pub fn owner<T: Into<String>>(mut self, value: T) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn sale_kind<T: Into<SaleKind>>(mut self, value: T) -> Self {
        self.sale_kind = Some(value.into());
        self
    }

    pub fn asset_contract_address<T: Into<String>>(mut self, value: T) -> Self {
        self.asset_contract_address = Some(value.into());
        self
    }

    pub fn payment_token_address<T: Into<String>>(mut self, value: T) -> Self {
        self.payment_token_address = Some(value.into());
        self
    }

    pub fn is_english<T: Into<bool>>(mut self, value: T) -> Self {
        self.is_english = Some(value.into());
        self
    }

    pub fn is_expired<T: Into<bool>>(mut self, value: T) -> Self {
        self.is_expired = Some(value.into());
        self
    }

    pub fn bundled<T: Into<bool>>(mut self, value: T) -> Self {
        self.bundled = Some(value.into());
        self
    }

    pub fn include_invalid<T: Into<bool>>(mut self, value: T) -> Self {
        self.include_invalid = Some(value.into());
        self
    }

    pub fn token_id<T: Into<String>>(mut self, value: T) -> Self {
        self.token_id = Some(value.into());
        self
    }

    pub fn token_ids<T: Into<Vec<String>>>(mut self, value: T) -> Self {
        self.token_ids = Some(value.into());
        self
    }

    pub fn listed_after<T: Into<String>>(mut self, value: T) -> Self {
        self.listed_after = Some(value.into());
        self
    }

    pub fn listed_before<T: Into<String>>(mut self, value: T) -> Self {
        self.listed_before = Some(value.into());
        self
    }

    pub fn limit<T: Into<u32>>(mut self, value: T) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn offset<T: Into<u32>>(mut self, value: T) -> Self {
        self.offset = Some(value.into());
        self
    }

    pub fn page<T: Into<u32>>(mut self, value: T) -> Self {
        self.page = Some(value.into());
        self
    }
}

impl Default for OrderQuery {
    fn default() -> Self {
        Self {
            owner: None,
            sale_kind: None,
            asset_contract_address: None,
            payment_token_address: None,
            is_english: None,
            is_expired: None,
            bundled: None,
            include_invalid: None,
            token_id: None,
            token_ids: None,
            listed_after: None,
            listed_before: None,
            limit: Some(20),
            offset: Some(0),
            page: Some(0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OpenSeaAssetsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contract_addresses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl OpenSeaAssetsQuery {
    pub fn owner<T: Into<String>>(mut self, value: T) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn asset_contract_address<T: Into<String>>(mut self, value: T) -> Self {
        self.asset_contract_address = Some(value.into());
        self
    }

    pub fn asset_contract_addresses<T: Into<Vec<String>>>(mut self, value: T) -> Self {
        self.asset_contract_addresses = Some(value.into());
        self
    }

    pub fn token_ids<T: Into<Vec<String>>>(mut self, value: T) -> Self {
        self.token_ids = Some(value.into());
        self
    }

    pub fn search<T: Into<String>>(mut self, value: T) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn order_by<T: Into<String>>(mut self, value: T) -> Self {
        self.order_by = Some(value.into());
        self
    }

    pub fn order_direction<T: Into<String>>(mut self, value: T) -> Self {
        self.order_direction = Some(value.into());
        self
    }

    pub fn limit<T: Into<u32>>(mut self, value: T) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn offset<T: Into<u32>>(mut self, value: T) -> Self {
        self.offset = Some(value.into());
        self
    }

    pub fn page<T: Into<u32>>(mut self, value: T) -> Self {
        self.page = Some(value.into());
        self
    }
}

impl Default for OpenSeaAssetsQuery {
    fn default() -> Self {
        Self {
            owner: None,
            asset_contract_address: None,
            asset_contract_addresses: None,
            token_ids: None,
            search: None,
            order_by: None,
            order_direction: None,
            limit: Some(20),
            offset: Some(0),
            page: Some(0),
        }
    }
}

/// A Query for a single Asset
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OpenSeaAssetQuery {
    pub token_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<u32>,
}

impl OpenSeaAssetQuery {
    pub fn new(token_address: impl Into<String>) -> Self {
        Self {
            token_address: token_address.into(),
            token_id: None,
        }
    }

    pub fn with_token_id(token_address: impl Into<String>, token_id: u32) -> Self {
        Self {
            token_address: token_address.into(),
            token_id: Some(token_id),
        }
    }
}

impl<T: Into<String>> From<T> for OpenSeaAssetQuery {
    fn from(token_address: T) -> Self {
        Self::new(token_address)
    }
}

/// Query interface for Bundles
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OpenSeaAssetBundleQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_sale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl OpenSeaAssetBundleQuery {
    pub fn asset_contract_address<T: Into<String>>(mut self, value: T) -> Self {
        self.asset_contract_address = Some(value.into());
        self
    }

    pub fn token_ids<T: Into<Vec<String>>>(mut self, value: T) -> Self {
        self.token_ids = Some(value.into());
        self
    }

    pub fn on_sale<T: Into<bool>>(mut self, value: T) -> Self {
        self.on_sale = Some(value.into());
        self
    }

    pub fn owner<T: Into<String>>(mut self, value: T) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn offset<T: Into<u32>>(mut self, value: T) -> Self {
        self.offset = Some(value.into());
        self
    }

    pub fn limit<T: Into<u32>>(mut self, value: T) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn search<T: Into<String>>(mut self, value: T) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn page<T: Into<u32>>(mut self, value: T) -> Self {
        self.page = Some(value.into());
        self
    }
}

impl Default for OpenSeaAssetBundleQuery {
    fn default() -> Self {
        Self {
            asset_contract_address: None,
            token_ids: None,
            on_sale: None,
            limit: Some(20),
            offset: Some(0),
            page: Some(0),
            owner: None,
            search: None,
        }
    }
}

/// Query interface for Collections
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OpenSeaAssetCollectionQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl OpenSeaAssetCollectionQuery {
    pub fn asset_owner<T: Into<String>>(mut self, value: T) -> Self {
        self.asset_owner = Some(value.into());
        self
    }

    pub fn offset<T: Into<u32>>(mut self, value: T) -> Self {
        self.offset = Some(value.into());
        self
    }

    pub fn limit<T: Into<u32>>(mut self, value: T) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn page<T: Into<u32>>(mut self, value: T) -> Self {
        self.page = Some(value.into());
        self
    }
}

impl Default for OpenSeaAssetCollectionQuery {
    fn default() -> Self {
        Self {
            asset_owner: None,
            limit: Some(20),
            offset: Some(0),
            page: Some(0),
        }
    }
}

/// Query interface for Collections
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OpenSeaEventsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contract_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventTypeQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_opensea: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auction_type: Option<AuctionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl OpenSeaEventsQuery {
    pub fn asset_contract_address<T: Into<String>>(mut self, value: T) -> Self {
        self.asset_contract_address = Some(value.into());
        self
    }

    pub fn collection_slug<T: Into<String>>(mut self, value: T) -> Self {
        self.collection_slug = Some(value.into());
        self
    }

    pub fn token_id<T: Into<String>>(mut self, value: T) -> Self {
        self.token_id = Some(value.into());
        self
    }

    pub fn account_address<T: Into<String>>(mut self, value: T) -> Self {
        self.account_address = Some(value.into());
        self
    }

    pub fn event_type<T: Into<EventTypeQuery>>(mut self, value: T) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn only_opensea<T: Into<bool>>(mut self, value: T) -> Self {
        self.only_opensea = Some(value.into());
        self
    }

    pub fn auction_type<T: Into<AuctionType>>(mut self, value: T) -> Self {
        self.auction_type = Some(value.into());
        self
    }

    pub fn offset<T: Into<u32>>(mut self, value: T) -> Self {
        self.offset = Some(value.into());
        self
    }

    pub fn limit<T: Into<u32>>(mut self, value: T) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn occurred_before<T: Into<String>>(mut self, value: T) -> Self {
        self.occurred_before = Some(value.into());
        self
    }

    pub fn occurred_after<T: Into<String>>(mut self, value: T) -> Self {
        self.occurred_after = Some(value.into());
        self
    }

    pub fn page<T: Into<u32>>(mut self, value: T) -> Self {
        self.page = Some(value.into());
        self
    }
}

impl Default for OpenSeaEventsQuery {
    fn default() -> Self {
        Self {
            asset_contract_address: None,
            collection_slug: None,
            token_id: None,
            account_address: None,
            event_type: None,
            only_opensea: None,
            limit: Some(20),
            occurred_before: None,
            offset: Some(0),
            page: Some(0),
            auction_type: None,
            occurred_after: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventTypeQuery {
    Created,
    Successful,
    Cancelled,
    BidEntered,
    BidWithdrawn,
    Transfer,
    Approve,
}

/// Query interface for Fungible Assets  Partial<OpenSeaFungibleToken>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OpenSeaFungibleTokenQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl OpenSeaFungibleTokenQuery {
    pub fn limit<T: Into<u32>>(mut self, value: T) -> Self {
        self.limit = Some(value.into());
        self
    }

    pub fn offset<T: Into<u32>>(mut self, value: T) -> Self {
        self.offset = Some(value.into());
        self
    }

    pub fn symbol<T: Into<i32>>(mut self, value: T) -> Self {
        self.symbol = Some(value.into());
        self
    }

    pub fn page<T: Into<u32>>(mut self, value: T) -> Self {
        self.page = Some(value.into());
        self
    }
}

impl Default for OpenSeaFungibleTokenQuery {
    fn default() -> Self {
        Self {
            limit: Some(20),
            offset: Some(0),
            symbol: None,
            page: Some(0),
        }
    }
}
