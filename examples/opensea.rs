use nftscape::opensea::query::OpenSeaAssetsQuery;
use nftscape::ApiClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = ApiClient::builder()
        .build(nftscape::opensea::API_BASE_MAINNET)
        .unwrap();

    let _asset = client
        .get_asset("0xb47e3cd837ddf8e4c57f05d70ab865de6e193bbb")
        .await?;

    let _assets = client.get_assets(&OpenSeaAssetsQuery::default()).await?;

    let _orders = client.get_orders(&Default::default()).await?;

    let _tokens = client.get_payment_tokens(&Default::default()).await?;

    let _bundle = client.get_bundles(&Default::default()).await?;

    Ok(())
}
