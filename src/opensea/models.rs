use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleList {
    pub bundles: Vec<OpenSeaAssetBundle>,
    pub estimated_count: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSeaAssetBundle {
    pub maker: CreatorRef,
    pub slug: String,
    pub assets: Vec<OpenSeaAsset>,
    pub name: String,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub asset_contract: Option<AssetContract>,
    pub permalink: String,
    pub sell_orders: Option<Vec<Order>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetList {
    pub assets: Vec<OpenSeaAsset>,
    pub estimated_count: Option<u64>,
}

/// The primary object in the OpenSea API is the asset, which represents a
/// unique digital item whose ownership is managed by the blockchain. The below
/// CryptoSaga hero is an example of an asset shown on OpenSea.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSeaAsset {
    pub id: Option<i64>,
    /// The token ID of the ERC721 asset
    pub token_id: Option<String>,
    /// Name of the item
    pub num_sales: i64,
    /// The background color to be displayed with the item
    pub background_color: Option<String>,
    /// An image for the item
    pub image_url: String,
    pub image_preview_url: Option<String>,
    pub image_thumbnail_url: Option<String>,
    pub image_original_url: Option<String>,
    pub animation_url: Option<String>,
    pub animation_original_url: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    /// External link to the original website for the item
    pub external_link: Option<String>,
    /// Dictionary of data on the contract itself
    pub asset_contract: AssetContract,
    pub permalink: String,
    pub collection: Collection,
    pub decimals: Option<i64>,
    pub token_metadata: Option<String>,
    /// Dictionary of data on the owner
    pub owner: OpenSeaAccount,
    pub sell_orders: Option<Vec<Order>>,
    pub creator: Option<OpenSeaAccount>,
    /// A list of traits associated with the item
    pub traits: Option<Vec<Trait>>,
    pub last_sale: Option<LastSale>,
    pub top_bid: Option<::serde_json::Value>,
    pub listing_date: Option<::serde_json::Value>,
    pub is_presale: Option<bool>,
    pub transfer_fee_payment_token: Option<::serde_json::Value>,
    pub transfer_fee: Option<::serde_json::Value>,
}

/// Asset contracts contain data about the contract itself, such as the
/// CryptoKitties contract or the CryptoFighters contract. Here are the field
/// associated with an asset contract:
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssetContract {
    /// Address of the asset contract
    pub address: String,
    pub asset_contract_type: String,
    pub created_date: String,
    /// Name of the asset contract
    pub name: String,
    pub nft_version: Option<String>,
    pub opensea_version: Option<String>,
    pub owner: Option<i64>,
    pub schema_name: String,
    /// Symbol, such as CKITTY
    pub symbol: String,
    pub total_supply: Option<String>,
    /// Description of the asset contract
    pub description: Option<String>,
    /// Link to the original website for this contract
    pub external_link: Option<String>,
    /// Image associated with the asset contract
    pub image_url: Option<String>,
    pub default_to_fiat: bool,
    pub dev_buyer_fee_basis_points: i64,
    pub dev_seller_fee_basis_points: i64,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: i64,
    pub opensea_seller_fee_basis_points: i64,
    pub buyer_fee_basis_points: i64,
    pub seller_fee_basis_points: i64,
    pub payout_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrimaryAssetContract {
    pub address: String,
    pub asset_contract_type: String,
    pub created_date: String,
    pub name: String,
    pub nft_version: Option<String>,
    pub opensea_version: ::serde_json::Value,
    pub owner: Option<i64>,
    pub schema_name: String,
    pub symbol: String,
    pub total_supply: String,
    pub description: Option<String>,
    pub external_link: Option<String>,
    pub image_url: Option<String>,
    pub default_to_fiat: bool,
    pub dev_buyer_fee_basis_points: i64,
    pub dev_seller_fee_basis_points: i64,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: i64,
    pub opensea_seller_fee_basis_points: i64,
    pub buyer_fee_basis_points: i64,
    pub seller_fee_basis_points: i64,
    pub payout_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionList {
    pub collections: Vec<OpenSeaAssetCollection>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSeaAssetCollection {
    /// Contains info about the smart contracts belonging to that collection.
    /// For example, ERC-1155 contracts maybe have multiple collections all
    /// referencing the same contract, but many ERC-721 contracts may all belong
    /// to the same collection (dapp).
    pub primary_asset_contracts: Vec<PrimaryAssetContract>,
    pub traits: Traits,
    pub stats: Stats,
    #[serde(flatten)]
    pub collection: Collection,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub banner_image_url: Option<String>,
    pub chat_url: Option<String>,
    pub created_date: String,
    pub default_to_fiat: bool,
    pub description: Option<String>,
    pub dev_buyer_fee_basis_points: String,
    pub dev_seller_fee_basis_points: String,
    pub discord_url: Option<String>,
    pub display_data: DisplayData,
    pub external_url: Option<String>,
    pub featured: bool,
    pub featured_image_url: Option<String>,
    pub hidden: bool,
    pub safelist_request_status: String,
    pub image_url: Option<String>,
    pub is_subject_to_whitelist: bool,
    pub large_image_url: Option<String>,
    pub medium_username: Option<String>,
    pub name: String,
    pub only_proxied_transfers: bool,
    pub opensea_buyer_fee_basis_points: String,
    pub opensea_seller_fee_basis_points: String,
    pub payout_address: Option<String>,
    pub require_email: bool,
    pub short_description: Option<String>,
    pub slug: String,
    pub telegram_url: Option<String>,
    pub twitter_username: Option<String>,
    pub instagram_username: Option<String>,
    pub wiki_url: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayData {
    pub card_display_style: String,
    #[serde(default)]
    pub images: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSeaUser {
    pub username: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBook {
    pub orders: Vec<Order>,
    pub count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub created_date: String,
    pub closing_date: Option<String>,
    pub closing_extendable: bool,
    pub expiration_time: i64,
    pub listing_time: i64,
    pub order_hash: String,
    pub metadata: ExchangeMetadata,
    pub exchange: String,
    pub maker: CreatorRef,
    pub taker: CreatorRef,
    pub current_price: String,
    pub current_bounty: String,
    pub bounty_multiple: String,
    pub maker_relayer_fee: String,
    pub taker_relayer_fee: String,
    pub maker_protocol_fee: String,
    pub taker_protocol_fee: String,
    pub maker_referrer_fee: String,
    pub fee_recipient: CreatorRef,
    pub fee_method: i64,
    pub side: i64,
    pub sale_kind: i64,
    pub target: String,
    pub how_to_call: i64,
    pub calldata: String,
    pub replacement_pattern: String,
    pub static_target: String,
    pub static_extradata: String,
    pub payment_token: String,
    pub payment_token_contract: PaymentTokenContract,
    pub base_price: String,
    pub extra: String,
    pub quantity: String,
    pub salt: String,
    pub v: i64,
    pub r: String,
    pub s: String,
    pub approved_on_chain: bool,
    pub cancelled: bool,
    pub finalized: bool,
    pub marked_invalid: bool,
    pub prefixed_hash: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExchangeMetadata {
    Asset(ExchangeMetadataForAsset),
    Bundle(ExchangeMetadataForBundle),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeMetadataForAsset {
    pub asset: WyvernAsset,
    pub schema: WyvernSchemaName,
    pub referrer_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeMetadataForBundle {
    pub bundle: WyvernBundle,
    pub referrer_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WyvernBundle {
    pub assets: Vec<WyvernAsset>,
    pub schemas: Vec<WyvernSchemaName>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub external_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WyvernAsset {
    NFT(WyvernNFTAsset),
    FT(WyvernFTAsset),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WyvernNFTAsset {
    pub id: String,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WyvernFTAsset {
    pub id: Option<String>,
    pub address: String,
    pub quantity: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub token_id: String,
    pub decimals: Option<u32>,
    pub name: Option<String>,
}

/// Maker, Taker, FeeRecipient
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatorRef {
    pub user: serde_json::Value,
    pub profile_img_url: String,
    pub address: String,
    pub config: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentTokenContract {
    pub id: i64,
    pub symbol: String,
    pub address: String,
    pub image_url: String,
    pub name: String,
    pub decimals: i64,
    pub eth_price: String,
    pub usd_price: String,
}

/// Accounts represent wallet addresses and associated usernames, if the owner
/// entered one on OpenSea. Here's an overview of the fields contained in an
/// account:
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSeaAccount {
    /// An object containing username, a String for the the OpenSea username
    /// associated with the account. Will be null if the account owner has not
    /// yet set a username on OpenSea.
    pub user: Option<OpenSeaUser>,
    /// An auto-generated profile picture to use for this wallet address.
    /// To get the user's Ethmoji avatar, use the [Ethmoji SDK](https://ethmoji.io/).
    pub profile_img_url: String,
    /// The Ethereum wallet address that uniquely identifies this account.
    pub address: String,
    /// A String representing public configuration options on the user's
    /// account, including affiliate and affiliate_requested for OpenSea
    /// affiliates and users waiting to be accepted as affiliates.
    pub config: String,
}

/// Traits are special properties on the item, that can either be numbers or
/// Strings. Below is an example of how OpenSea displays the traits for a
/// specific item.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trait {
    /// The name of the trait
    pub trait_type: String,
    /// The value of this trait
    pub value: Option<serde_json::Value>,
    /// How this trait will be displayed (options are number, boost_percentage,
    /// boost_number, and date).
    /// See the [adding metadata section](https://docs.opensea.io/docs/2-adding-metadata) for more details
    pub display_type: Option<String>,
    // `"1"`
    pub max_value: Option<String>,
    pub trait_count: i64,
    pub order: ::serde_json::Value,
}

// AssetEvent
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastSale {
    pub asset: Asset,
    pub asset_bundle: ::serde_json::Value,
    pub event_type: String,
    pub event_timestamp: String,
    pub auction_type: Option<AuctionType>,
    pub total_price: String,
    pub payment_token: Option<OpenSeaFungibleToken>,
    pub transaction: Option<Transaction>,
    pub created_date: String,
    pub quantity: String,
}

pub type FungibleTokenList = Vec<OpenSeaFungibleToken>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenSeaFungibleToken {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u32,
    pub address: String,
    pub image_url: Option<String>,
    pub eth_price: Option<String>,
    pub usd_price: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transaction {
    pub block_hash: String,
    pub block_number: String,
    pub from_account: OpenSeaAccount,
    pub id: i64,
    pub timestamp: String,
    pub to_account: OpenSeaAccount,
    pub transaction_hash: String,
    pub transaction_index: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Traits {
    #[serde(rename = "Vitality")]
    pub vitality: Option<Range>,
    #[serde(rename = " Technology")]
    pub technology: Option<Range>,
    #[serde(rename = "Aggressiveness")]
    pub aggressiveness: Option<Range>,
    pub high: Option<Range>,
    #[serde(rename = "Serial No.")]
    pub serial_no: Option<Range>,
    #[serde(rename = "1")]
    pub n1: Option<Range>,
    #[serde(rename = "Cuteness")]
    pub cuteness: Option<Range>,
    #[serde(rename = "Pixel")]
    pub pixel: Option<Range>,
    #[serde(rename = "Chubs")]
    pub chubs: Option<Range>,
    #[serde(rename = "12116")]
    pub n12116: Option<Range>,
    #[serde(rename = "554356")]
    pub n554356: Option<Range>,
    pub cover: Option<Range>,
    #[serde(rename = "Supply Number")]
    pub supply_number: Option<Range>,
    #[serde(rename = "Portrait")]
    pub portrait: Option<Range>,
    #[serde(rename = "High")]
    pub high2: Option<Range>,
    pub dance: Option<Range>,
    pub jokes: Option<Range>,
    pub funny: Option<Range>,
    #[serde(rename = "99")]
    pub n99: Option<Range>,
    #[serde(rename = "+1 bacon")]
    pub bacon: Option<Range>,
    #[serde(rename = "Drip")]
    pub drip: Option<Range>,
    #[serde(rename = "Juice")]
    pub juice: Option<Range>,
    #[serde(rename = "Power")]
    pub power: Option<Range>,
    #[serde(rename = "Speed")]
    pub speed: Option<Range>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

// Alias for Vitality, Technology, Aggressiveness, High, SerialNo, N1, Cuteness,
// Pixel, Chubs, N12116, N554356, Cover, SupplyNumber, Portrait, High2, Dance,
// Jokes, Funny, N99, Bacon, Drip, Juice, Power, Speed
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub min: i64,
    pub max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub one_day_volume: i64,
    pub one_day_change: i64,
    pub one_day_sales: i64,
    pub one_day_average_price: i64,
    pub seven_day_volume: i64,
    pub seven_day_change: i64,
    pub seven_day_sales: i64,
    pub seven_day_average_price: i64,
    pub thirty_day_volume: i64,
    pub thirty_day_change: i64,
    pub thirty_day_sales: i64,
    pub thirty_day_average_price: i64,
    pub total_volume: i64,
    pub total_sales: i64,
    pub total_supply: i64,
    pub count: i64,
    pub num_owners: i64,
    pub average_price: i64,
    pub num_reports: i64,
    pub market_cap: i64,
    pub floor_price: i64,
}

/// Wyvern order side: buy or sell.
#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum OrderSide {
    Buy = 0,
    Sell = 1,
}

/// Wyvern fee method
#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum FeeMethod {
    /// Charge maker fee to seller and charge taker fee to buyer.
    ProtocolFee = 0,
    /// Maker fees are deducted from the token amount that the maker receives.
    /// Taker fees are extra tokens that must be paid by the taker.
    SplitFee = 1,
}

///  Wyvern: type of sale. Fixed or Dutch auction
#[derive(Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum SaleKind {
    FixedPrice = 0,
    DutchAuction = 1,
}

#[derive(Copy, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuctionType {
    Dutch,
    English,
    MinPrice,
}

/// Types of asset contracts
/// Given by the asset_contract_type in the OpenSea API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetContractType {
    #[serde(rename = "fungible")]
    Fungible,
    #[serde(rename = "semi-fungible")]
    SemiFungible,
    #[serde(rename = "non-fungible")]
    NonFungible,
    #[serde(rename = "unknown")]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WyvernSchemaName {
    #[serde(rename = "ERC20")]
    ERC20,
    #[serde(rename = "ERC721")]
    ERC721,
    #[serde(rename = "ERC1155")]
    ERC1155,
    #[serde(rename = "Enjin")]
    LegacyEnjin,
    #[serde(rename = "ENSShortNameAuction")]
    ENSShortNameAuction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WyvernAssetLocation {
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "proxy")]
    Proxy,
    #[serde(rename = "other")]
    Other,
}

/// The NFT version that this contract uses.
/// ERC721 versions are:
/// 1.0: CryptoKitties and early 721s, which lack approve-all and
///      have problems calling `transferFrom` from the owner's account.
/// 2.0: CryptoSaga and others that lack `transferFrom` and have
///      `takeOwnership` instead
/// 3.0: The current OpenZeppelin standard:
///      https://github.com/OpenZeppelin/openzeppelin-solidity/blob/master/contracts/token/ERC721/ERC721.sol
/// Special cases:
/// locked: When the transfer function has been locked by the dev
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenStandardVersion {
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "1155-1.0")]
    Enjin,
    #[serde(rename = "1.0")]
    ERC721v1,
    #[serde(rename = "2.0")]
    ERC721v2,
    #[serde(rename = "3.0")]
    ERC721v3,
}

///  Events emitted by the SDK. There are five types:
///  1. Transaction events, which tell you when a new transaction was
///     created, confirmed, denied, or failed.
///  2. pre-transaction events, which are named (like "WrapEth") and indicate
///     that Web3 is asking for a signature on a transaction that needs to occur
/// before     an order is made or fulfilled. This includes approval events and
/// account     initialization.
///  3. Basic actions: matching, cancelling, and creating orders.
///     The "CreateOrder" event fires when a signature is being prompted
///     to create an off-chain order. The "OrderDenied" event fires when a
/// signature     request is denied by the user.
///  4. The "TransferAll" event, which fires when a user is about to directly
///     transfer one or more assets to another account
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    TransactionCreated,
    TransactionConfirmed,
    TransactionDenied,
    TransactionFailed,
    InitializeAccount,
    WrapEth,
    UnwrapWeth,
    ApproveCurrency,
    ApproveAsset,
    ApproveAllAssets,
    UnapproveCurrency,
    MatchOrders,
    CancelOrder,
    ApproveOrder,
    CreateOrder,
    OrderDenied,
    TransferAll,
    TransferOne,
    WrapAssets,
    UnwrapAssets,
    LiquidateAssets,
    PurchaseAssets,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventData {
    pub account_address: Option<String>,
    pub to_address: Option<String>,
    pub proxy_address: Option<String>,
    pub amount: Option<u64>,
    pub contract_address: Option<String>,
    pub assets: Option<Vec<WyvernAsset>>,
    pub asset: Option<WyvernAsset>,
    pub transaction_hash: Option<String>,
    pub event: Option<EventType>,
    pub error: Option<String>,
    pub order: Option<Order>,
    pub buy: Option<Order>,
    pub sell: Option<Order>,
    pub match_metadata: Option<String>,
}
