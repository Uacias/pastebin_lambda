use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use std::sync::Arc;

pub async fn get_dynamodb_client() -> Result<Arc<Client>, aws_sdk_dynamodb::Error> {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);
    Ok(Arc::new(client))
}
