use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Activities {
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<Activity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "left")]
    pub left: Box<OrderActivityMatchSide>,
    #[serde(rename = "right")]
    pub right: Box<OrderActivityMatchSide>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "priceUsd", skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(rename = "logIndex")]
    pub log_index: i32,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "make")]
    pub make: Box<AssetType>,
    #[serde(rename = "take")]
    pub take: Box<AssetType>,
    #[serde(rename = "from")]
    pub from: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityContinuation {
    #[serde(rename = "afterDate")]
    pub after_date: String,
    #[serde(rename = "afterId")]
    pub after_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityFilter {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
    #[serde(rename = "users")]
    pub users: Vec<String>,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActivityTypes {
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "MINT")]
    Mint,
    #[serde(rename = "BURN")]
    Burn,
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "MATCH")]
    _Match,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityFilterAll {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityFilterByCollection {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityFilterByItem {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityFilterByUser {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "users")]
    pub users: Vec<String>,
    #[serde(rename = "types")]
    pub types: Vec<ActivityFilterTypes>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActivityFilterTypes {
    #[serde(rename = "TRANSFER_FROM")]
    TransferFrom,
    #[serde(rename = "TRANSFER_TO")]
    TransferTo,
    #[serde(rename = "MINT")]
    Mint,
    #[serde(rename = "BURN")]
    Burn,
    #[serde(rename = "MAKE_BID")]
    MakeBid,
    #[serde(rename = "GET_BID")]
    GetBid,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AggregationData {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "sum")]
    pub sum: String,
    #[serde(rename = "count")]
    pub count: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AggregationSource {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "RARIBLE")]
    Rarible,
    #[serde(rename = "OPEN_SEA")]
    Opensea,
}

impl fmt::Display for AggregationSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AggregationSource::All => f.write_str("ALL"),
            AggregationSource::Rarible => f.write_str("RARIBLE"),
            AggregationSource::Opensea => f.write_str("OPEN_SEA"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "assetType")]
    pub asset_type: Box<AssetType>,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "assetclass")]
pub enum AssetType {
    #[serde(rename = "ERC1155")]
    Erc1155 {
        #[serde(rename = "contract")]
        contract: String,
        #[serde(rename = "tokenId")]
        token_id: String,
    },
    #[serde(rename = "ERC1155_LAZY")]
    Erc1155Lazy {
        #[serde(rename = "contract")]
        contract: String,
        #[serde(rename = "tokenId")]
        token_id: String,
        #[serde(rename = "uri")]
        uri: String,
        #[serde(rename = "supply")]
        supply: String,
        #[serde(rename = "creators")]
        creators: Vec<Part>,
        #[serde(rename = "royalties")]
        royalties: Vec<Part>,
        #[serde(rename = "signatures")]
        signatures: Vec<String>,
    },
    #[serde(rename = "ERC20")]
    Erc20 {
        #[serde(rename = "contract")]
        contract: String,
    },
    #[serde(rename = "ERC721")]
    Erc721 {
        #[serde(rename = "contract")]
        contract: String,
        #[serde(rename = "tokenId")]
        token_id: String,
    },
    #[serde(rename = "ERC721_LAZY")]
    Erc721Lazy {
        #[serde(rename = "contract")]
        contract: String,
        #[serde(rename = "tokenId")]
        token_id: String,
        #[serde(rename = "uri")]
        uri: String,
        #[serde(rename = "creators")]
        creators: Vec<Part>,
        #[serde(rename = "royalties")]
        royalties: Vec<Part>,
        #[serde(rename = "signatures")]
        signatures: Vec<String>,
    },
    #[serde(rename = "ETH")]
    Eth,
    #[serde(rename = "FLOW")]
    Flow,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Burn {
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "nonce")]
    pub nonce: i64,
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(rename = "input")]
    pub input: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrencyRate {
    #[serde(rename = "fromCurrencyId")]
    pub from_currency_id: String,
    #[serde(rename = "toCurrencyId")]
    pub to_currency_id: String,
    #[serde(rename = "rate")]
    pub rate: String,
    #[serde(rename = "date")]
    pub date: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Eip712Domain {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "verifyingContract")]
    pub verifying_contract: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Eip712SignMessage {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "domain")]
    pub domain: Box<Eip712Domain>,
    #[serde(rename = "struct")]
    pub _struct: serde_json::Value,
    #[serde(rename = "structType")]
    pub struct_type: String,
    #[serde(rename = "types")]
    pub types: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedOrder {
    #[serde(
        rename = "transferProxyAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_proxy_address: Option<String>,
    #[serde(rename = "signMessage")]
    pub sign_message: Box<SignMessage>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncodedOrderData {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc1155AssetType {
    #[serde(rename = "assetClass")]
    pub asset_class: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc1155LazyAssetType {
    #[serde(rename = "assetClass")]
    pub asset_class: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "supply")]
    pub supply: String,
    #[serde(rename = "creators")]
    pub creators: Vec<Part>,
    #[serde(rename = "royalties")]
    pub royalties: Vec<Part>,
    #[serde(rename = "signatures")]
    pub signatures: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc20AssetType {
    #[serde(rename = "assetClass")]
    pub asset_class: String,
    #[serde(rename = "contract")]
    pub contract: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc20Balance {
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "balance")]
    pub balance: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Erc20BalanceEvent {
    #[serde(rename = "UPDATE")]
    Erc20BalanceUpdateEvent {
        #[serde(rename = "balance")]
        balance: Box<Erc20Balance>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc20BalanceUpdateEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "balance")]
    pub balance: Box<Erc20Balance>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc20DecimalBalance {
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "decimalBalance")]
    pub decimal_balance: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc20Token {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc721AssetType {
    #[serde(rename = "assetClass")]
    pub asset_class: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Erc721LazyAssetType {
    #[serde(rename = "assetClass")]
    pub asset_class: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "creators")]
    pub creators: Vec<Part>,
    #[serde(rename = "royalties")]
    pub royalties: Vec<Part>,
    #[serde(rename = "signatures")]
    pub signatures: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EthAssetType {
    #[serde(rename = "assetClass")]
    pub asset_class: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlowAssetType {
    #[serde(rename = "assetClass")]
    pub asset_class: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvertOrderForm {
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "salt")]
    pub salt: String,
    #[serde(rename = "originFees")]
    pub origin_fees: Vec<Part>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemBuy {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "fill")]
    pub fill: String,
    #[serde(rename = "buyer", skip_serializing_if = "Option::is_none")]
    pub buyer: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ItemHistory {
    #[serde(rename = "ROYALTY")]
    ItemRoyalty {
        #[serde(rename = "royalties")]
        royalties: Vec<Part>,
    },
    #[serde(rename = "TRANSFER")]
    ItemTransfer {
        #[serde(rename = "from")]
        from: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemRoyalty {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "royalties")]
    pub royalties: Vec<Part>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemTransfer {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "from")]
    pub from: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LazyErc1155 {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "supply")]
    pub supply: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LazyErc721 {
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LazyNft {
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "uri")]
    pub uri: String,
    #[serde(rename = "creators")]
    pub creators: Vec<Part>,
    #[serde(rename = "royalties")]
    pub royalties: Vec<Part>,
    #[serde(rename = "signatures")]
    pub signatures: Vec<String>,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "supply")]
    pub supply: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Lock {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "unlockDate", skip_serializing_if = "Option::is_none")]
    pub unlock_date: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LockForm {
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogEvent {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "topic")]
    pub topic: String,
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<i64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "CONFIRMED")]
    Confirmed,
    #[serde(rename = "REVERTED")]
    Reverted,
    #[serde(rename = "DROPPED")]
    Dropped,
    #[serde(rename = "INACTIVE")]
    Inactive,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mint {
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftActivities {
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<NftActivity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftActivity {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(rename = "logIndex")]
    pub log_index: i32,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "from")]
    pub from: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftActivityFilter {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
    #[serde(rename = "users")]
    pub users: Vec<String>,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftActivityFilterAll {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftActivityFilterByCollection {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftActivityFilterByItem {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "types")]
    pub types: Vec<ActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftActivityFilterByUser {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "users")]
    pub users: Vec<String>,
    #[serde(rename = "types")]
    pub types: Vec<ActivityFilterByUserTypes>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActivityFilterByUserTypes {
    #[serde(rename = "TRANSFER_FROM")]
    TransferFrom,
    #[serde(rename = "TRANSFER_TO")]
    TransferTo,
    #[serde(rename = "MINT")]
    Mint,
    #[serde(rename = "BURN")]
    Burn,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftCollection {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub _type: TokenType,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "features")]
    pub features: Vec<Features>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TokenType {
    #[serde(rename = "ERC721")]
    ERC721,
    #[serde(rename = "ERC1155")]
    ERC1155,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Features {
    #[serde(rename = "APPROVE_FOR_ALL")]
    ApproveForAll,
    #[serde(rename = "SET_URI_PREFIX")]
    SeturiPrefix,
    #[serde(rename = "BURN")]
    Burn,
    #[serde(rename = "MINT_WITH_ADDRESS")]
    MintWithAddress,
    #[serde(rename = "SECONDARY_SALE_FEES")]
    SecondarySaleFees,
    #[serde(rename = "MINT_AND_TRANSFER")]
    MintAndTransfer,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NftCollectionHistory {
    #[serde(rename = "COLLECTION_CREATE")]
    NftCreateCollection {
        #[serde(rename = "owner")]
        owner: String,
        #[serde(rename = "name")]
        name: String,
        #[serde(rename = "symbol")]
        symbol: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftCollections {
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "collections")]
    pub collections: Vec<NftCollection>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftCreateCollection {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftDeletedItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftDeletedOwnership {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "owner")]
    pub owner: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "creators")]
    pub creators: Vec<Part>,
    #[serde(rename = "supply")]
    pub supply: String,
    #[serde(rename = "lazySupply")]
    pub lazy_supply: String,
    #[serde(rename = "owners")]
    pub owners: Vec<String>,
    #[serde(rename = "royalties")]
    pub royalties: Vec<Part>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<Vec<ItemTransfer>>,
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<NftItemMeta>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemAttribute {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemDeleteEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "item")]
    pub item: Box<NftDeletedItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NftItemEvent {
    #[serde(rename = "DELETE")]
    NftItemDeleteEvent {
        #[serde(rename = "item")]
        item: Box<NftDeletedItem>,
    },
    #[serde(rename = "UPDATE")]
    NftItemUpdateEvent {
        #[serde(rename = "item")]
        item: Box<NftItem>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemFilter {
    #[serde(rename = "sort")]
    pub sort: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "showDeleted")]
    pub show_deleted: bool,
    #[serde(rename = "lastUpdatedFrom", skip_serializing_if = "Option::is_none")]
    pub last_updated_from: Option<String>,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "collection")]
    pub collection: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemFilterAll {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "showDeleted")]
    pub show_deleted: bool,
    #[serde(rename = "lastUpdatedFrom", skip_serializing_if = "Option::is_none")]
    pub last_updated_from: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemFilterByCollection {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "collection")]
    pub collection: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemFilterByCreator {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "creator")]
    pub creator: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemFilterByOwner {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "owner")]
    pub owner: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemMeta {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<NftItemAttribute>>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<NftMedia>>,
    #[serde(rename = "animation", skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<NftMedia>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItemUpdateEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "item")]
    pub item: Box<NftItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftItems {
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<NftItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftMedia {
    #[serde(rename = "url")]
    pub url: ::std::collections::HashMap<String, String>,
    #[serde(rename = "meta")]
    pub meta: ::std::collections::HashMap<String, NftMediaMeta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftMediaMeta {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NftMediaSize {
    #[serde(rename = "ORIGINAL")]
    Original,
    #[serde(rename = "BIG")]
    Big,
    #[serde(rename = "PREVIEW")]
    Preview,
}

impl fmt::Display for NftMediaSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Original => f.write_str("ORIGINAL"),
            Self::Big => f.write_str("BIG"),
            Self::Preview => f.write_str("PREVIEW"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderDeletedItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderDeletedOwnership {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "owner")]
    pub owner: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderItem {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "unlockable")]
    pub unlockable: bool,
    #[serde(rename = "creators")]
    pub creators: Vec<Part>,
    #[serde(rename = "supply")]
    pub supply: String,
    #[serde(rename = "lazySupply")]
    pub lazy_supply: String,
    #[serde(rename = "owners")]
    pub owners: Vec<String>,
    #[serde(rename = "royalties")]
    pub royalties: Vec<Part>,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "pending")]
    pub pending: Vec<ItemTransfer>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<NftItemMeta>>,
    #[serde(rename = "bestSellOrder", skip_serializing_if = "Option::is_none")]
    pub best_sell_order: Option<Box<Order>>,
    #[serde(rename = "bestBidOrder", skip_serializing_if = "Option::is_none")]
    pub best_bid_order: Option<Box<Order>>,
    #[serde(rename = "totalStock")]
    pub total_stock: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderItemDeleteEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "item")]
    pub item: Box<NftOrderDeletedItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NftOrderItemEvent {
    #[serde(rename = "DELETE")]
    NftOrderItemDeleteEvent {
        #[serde(rename = "item")]
        item: Box<NftOrderDeletedItem>,
    },
    #[serde(rename = "UPDATE")]
    NftOrderItemUpdateEvent {
        #[serde(rename = "item")]
        item: Box<NftOrderItem>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderItemUpdateEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "item")]
    pub item: Box<NftOrderItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderOwnership {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "creators", skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<Part>>,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "lazyValue")]
    pub lazy_value: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "pending")]
    pub pending: Vec<ItemHistory>,
    #[serde(rename = "bestSellOrder", skip_serializing_if = "Option::is_none")]
    pub best_sell_order: Option<Box<Order>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderOwnershipDeleteEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "ownership")]
    pub ownership: Box<NftOrderDeletedOwnership>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NftOrderOwnershipEvent {
    #[serde(rename = "DELETE")]
    NftOrderOwnershipDeleteEvent {
        #[serde(rename = "ownership")]
        ownership: Box<NftOrderDeletedOwnership>,
    },
    #[serde(rename = "UPDATE")]
    NftOrderOwnershipUpdateEvent {
        #[serde(rename = "ownership")]
        ownership: Box<NftOrderOwnership>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOrderOwnershipUpdateEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "ownership")]
    pub ownership: Box<NftOrderOwnership>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnership {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "creators")]
    pub creators: Vec<Part>,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "lazyValue")]
    pub lazy_value: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "pending")]
    pub pending: Vec<ItemHistory>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipDeleteEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "ownership")]
    pub ownership: Box<NftDeletedOwnership>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NftOwnershipEvent {
    #[serde(rename = "DELETE")]
    NftOwnershipDeleteEvent {
        #[serde(rename = "ownership")]
        ownership: Box<NftDeletedOwnership>,
    },
    #[serde(rename = "UPDATE")]
    NftOwnershipUpdateEvent {
        #[serde(rename = "ownership")]
        ownership: Box<NftOwnership>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipFilter {
    #[serde(rename = "sort")]
    pub sort: String,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipFilterAll {
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipFilterByCollection {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "collection")]
    pub collection: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipFilterByCreator {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "creator")]
    pub creator: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipFilterByItem {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipFilterByOwner {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "owner")]
    pub owner: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnershipUpdateEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "ownership")]
    pub ownership: Box<NftOwnership>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftOwnerships {
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "ownerships")]
    pub ownerships: Vec<NftOwnership>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftSignature {
    #[serde(rename = "v")]
    pub v: String,
    #[serde(rename = "r")]
    pub r: String,
    #[serde(rename = "s")]
    pub s: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftSort {
    #[serde(rename = "property")]
    pub property: String,
    #[serde(rename = "direction")]
    pub direction: Direction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NftTokenId {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "signature")]
    pub signature: Box<NftSignature>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "type")]
    pub _type: OrderType,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "taker", skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    #[serde(rename = "make")]
    pub make: Box<Asset>,
    #[serde(rename = "take")]
    pub take: Box<Asset>,
    #[serde(rename = "fill")]
    pub fill: String,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(rename = "makeStock")]
    pub make_stock: String,
    #[serde(rename = "cancelled")]
    pub cancelled: bool,
    #[serde(rename = "salt")]
    pub salt: String,
    #[serde(rename = "data")]
    pub data: Box<OrderData>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "lastUpdateAt")]
    pub last_update_at: String,
    #[serde(rename = "pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<Vec<OrderExchangeHistory>>,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "makeBalance", skip_serializing_if = "Option::is_none")]
    pub make_balance: Option<String>,
    #[serde(rename = "makePriceUsd", skip_serializing_if = "Option::is_none")]
    pub make_price_usd: Option<String>,
    #[serde(rename = "takePriceUsd", skip_serializing_if = "Option::is_none")]
    pub take_price_usd: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivities {
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "items")]
    pub items: Vec<OrderActivity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivity {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "source")]
    pub source: Source,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "left")]
    pub left: Box<OrderActivityMatchSide>,
    #[serde(rename = "right")]
    pub right: Box<OrderActivityMatchSide>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "priceUsd", skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(rename = "logIndex")]
    pub log_index: i32,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "make")]
    pub make: Box<AssetType>,
    #[serde(rename = "take")]
    pub take: Box<AssetType>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "RARIBLE")]
    Rarible,
    #[serde(rename = "OPEN_SEA")]
    Opensea,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityBid {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "make")]
    pub make: Box<Asset>,
    #[serde(rename = "take")]
    pub take: Box<Asset>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "priceUsd", skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityCancelBid {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "make")]
    pub make: Box<AssetType>,
    #[serde(rename = "take")]
    pub take: Box<AssetType>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(rename = "logIndex")]
    pub log_index: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityCancelList {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "make")]
    pub make: Box<AssetType>,
    #[serde(rename = "take")]
    pub take: Box<AssetType>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(rename = "logIndex")]
    pub log_index: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityFilter {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "types")]
    pub types: Vec<OrderActivityTypes>,
    #[serde(rename = "users")]
    pub users: Vec<String>,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderActivityTypes {
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "MATCH")]
    _Match,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityFilterAll {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "types")]
    pub types: Vec<OrderActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityFilterByCollection {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "types")]
    pub types: Vec<OrderActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityFilterByItem {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "types")]
    pub types: Vec<OrderActivityTypes>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityFilterByUser {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "users")]
    pub users: Vec<String>,
    #[serde(rename = "types")]
    pub types: Vec<OrderTypes>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderTypes {
    #[serde(rename = "MAKE_BID")]
    MakeBid,
    #[serde(rename = "GET_BID")]
    GetBid,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityList {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "make")]
    pub make: Box<Asset>,
    #[serde(rename = "take")]
    pub take: Box<Asset>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "priceUsd", skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityMatch {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "left")]
    pub left: Box<OrderActivityMatchSide>,
    #[serde(rename = "right")]
    pub right: Box<OrderActivityMatchSide>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "priceUsd", skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    #[serde(rename = "blockNumber")]
    pub block_number: i64,
    #[serde(rename = "logIndex")]
    pub log_index: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderActivityMatchSide {
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "hash")]
    pub hash: String,
    #[serde(rename = "asset")]
    pub asset: Box<Asset>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<OrderActivityType>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderActivityType {
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "BID")]
    Bid,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderBid {
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "status")]
    pub status: OrderBidStatus,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "taker", skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    #[serde(rename = "make")]
    pub make: Box<Asset>,
    #[serde(rename = "take")]
    pub take: Box<Asset>,
    #[serde(rename = "makeBalance", skip_serializing_if = "Option::is_none")]
    pub make_balance: Option<String>,
    #[serde(rename = "makePriceUsd", skip_serializing_if = "Option::is_none")]
    pub make_price_usd: Option<String>,
    #[serde(rename = "takePriceUsd", skip_serializing_if = "Option::is_none")]
    pub take_price_usd: Option<String>,
    #[serde(rename = "type")]
    pub _type: OrderType,
    #[serde(rename = "fill")]
    pub fill: String,
    #[serde(rename = "makeStock")]
    pub make_stock: String,
    #[serde(rename = "cancelled")]
    pub cancelled: bool,
    #[serde(rename = "salt")]
    pub salt: String,
    #[serde(rename = "data")]
    pub data: Box<OrderData>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderBidStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "HISTORICAL")]
    Historical,
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}
impl ToString for OrderBidStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => "ACTIVE".to_string(),
            Self::Filled => "FILLED".to_string(),
            Self::Historical => "HISTORICAL".to_string(),
            Self::Inactive => "INACTIVE".to_string(),
            Self::Cancelled => "CANCELLED".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderBidsPagination {
    #[serde(rename = "items")]
    pub items: Vec<OrderBid>,
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderCancel {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "datatype")]
pub enum OrderData {
    #[serde(rename = "LEGACY")]
    OrderDataLegacy {
        #[serde(rename = "fee")]
        fee: i32,
    },
    #[serde(rename = "RARIBLE_V2_DATA_V1")]
    OrderRaribleV2DataV1 {
        #[serde(rename = "payouts")]
        payouts: Vec<Part>,
        #[serde(rename = "originFees")]
        origin_fees: Vec<Part>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderDataLegacy {
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "fee")]
    pub fee: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OrderEvent {
    #[serde(rename = "UPDATE")]
    OrderUpdateEvent {
        #[serde(rename = "order")]
        order: Box<Order>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OrderExchangeHistory {
    #[serde(rename = "CANCEL")]
    OrderCancel {
        #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
        owner: Option<String>,
    },
    #[serde(rename = "ORDER_SIDE_MATCH")]
    OrderSideMatch {
        #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
        side: Option<OrderSide>,
        #[serde(rename = "fill")]
        fill: String,
        #[serde(rename = "taker", skip_serializing_if = "Option::is_none")]
        taker: Option<String>,
        #[serde(rename = "counterHash", skip_serializing_if = "Option::is_none")]
        counter_hash: Option<String>,
        #[serde(rename = "makeUsd", skip_serializing_if = "Option::is_none")]
        make_usd: Option<String>,
        #[serde(rename = "takeUsd", skip_serializing_if = "Option::is_none")]
        take_usd: Option<String>,
        #[serde(rename = "makePriceUsd", skip_serializing_if = "Option::is_none")]
        make_price_usd: Option<String>,
        #[serde(rename = "takePriceUsd", skip_serializing_if = "Option::is_none")]
        take_price_usd: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilter {
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(rename = "sort")]
    pub sort: Sort,
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "collection")]
    pub collection: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Sort {
    #[serde(rename = "LAST_UPDATE")]
    LastUpdate,
    #[serde(rename = "TAKE_PRICE_DESC")]
    TakePriceDesc,
    #[serde(rename = "MAKE_PRICE_ASC")]
    MakePriceAsc,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilterAll {
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilterBidByItem {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "maker", skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilterBidByMaker {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "maker")]
    pub maker: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilterSell {
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilterSellByCollection {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "collection")]
    pub collection: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilterSellByItem {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "contract")]
    pub contract: String,
    #[serde(rename = "tokenId")]
    pub token_id: String,
    #[serde(rename = "maker", skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderFilterSellByMaker {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "maker")]
    pub maker: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderForm {
    #[serde(rename = "type")]
    pub _type: OrderType,
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "taker", skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    #[serde(rename = "make")]
    pub make: Box<Asset>,
    #[serde(rename = "take")]
    pub take: Box<Asset>,
    #[serde(rename = "salt")]
    pub salt: String,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(rename = "data")]
    pub data: Box<OrderData>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderRaribleV2DataV1 {
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "payouts")]
    pub payouts: Vec<Part>,
    #[serde(rename = "originFees")]
    pub origin_fees: Vec<Part>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl ToString for OrderSide {
    fn to_string(&self) -> String {
        match self {
            Self::Left => String::from("LEFT"),
            Self::Right => String::from("RIGHT"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderSideMatch {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<OrderSide>,
    #[serde(rename = "fill")]
    pub fill: String,
    #[serde(rename = "taker", skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    #[serde(rename = "counterHash", skip_serializing_if = "Option::is_none")]
    pub counter_hash: Option<String>,
    #[serde(rename = "makeUsd", skip_serializing_if = "Option::is_none")]
    pub make_usd: Option<String>,
    #[serde(rename = "takeUsd", skip_serializing_if = "Option::is_none")]
    pub take_usd: Option<String>,
    #[serde(rename = "makePriceUsd", skip_serializing_if = "Option::is_none")]
    pub make_price_usd: Option<String>,
    #[serde(rename = "takePriceUsd", skip_serializing_if = "Option::is_none")]
    pub take_price_usd: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "RARIBLE_V1")]
    V1,
    #[serde(rename = "RARIBLE_V2")]
    V2,
}
impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            Self::V1 => String::from("RARIBLE_V1"),
            Self::V2 => String::from("RARIBLE_V2"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderUpdateEvent {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "order")]
    pub order: Box<Order>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrdersPagination {
    #[serde(rename = "orders")]
    pub orders: Vec<Order>,
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageNftOrderItem {
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "data")]
    pub data: Vec<NftOrderItem>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageNftOrderOwnershipItem {
    #[serde(rename = "continuation", skip_serializing_if = "Option::is_none")]
    pub continuation: Option<String>,
    #[serde(rename = "data")]
    pub data: Vec<NftOrderOwnership>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Part {
    #[serde(rename = "account")]
    pub account: String,
    #[serde(rename = "value")]
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareOrderTxForm {
    #[serde(rename = "maker")]
    pub maker: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "payouts")]
    pub payouts: Vec<Part>,
    #[serde(rename = "originFees")]
    pub origin_fees: Vec<Part>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrepareOrderTxResponse {
    #[serde(
        rename = "transferProxyAddress",
        skip_serializing_if = "Option::is_none"
    )]
    pub transfer_proxy_address: Option<String>,
    #[serde(rename = "asset")]
    pub asset: Box<Asset>,
    #[serde(rename = "transaction")]
    pub transaction: Box<PreparedOrderTx>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreparedOrderTx {
    #[serde(rename = "to")]
    pub to: String,
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignMessage {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "domain")]
    pub domain: Box<Eip712Domain>,
    #[serde(rename = "struct")]
    pub _struct: serde_json::Value,
    #[serde(rename = "structType")]
    pub struct_type: String,
    #[serde(rename = "types")]
    pub types: serde_json::Value,
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignatureForm {
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextSignMessage {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(rename = "@type")]
    pub _type: String,
    #[serde(rename = "from")]
    pub from: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnlockableEvent {
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "itemId")]
    pub item_id: String,
    #[serde(rename = "type")]
    pub _type: UnlockType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UnlockType {
    #[serde(rename = "LOCK_CREATED")]
    Created,
    #[serde(rename = "LOCK_UNLOCKED")]
    Unlocked,
}
