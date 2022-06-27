use aws_sdk_s3::Client;
use aws_types::sdk_config::SdkConfig;

/// Lists your buckets
pub async fn list_buckets(config: SdkConfig) -> Result<Vec<String>, aws_sdk_s3::Error> {
    let client = Client::new(&config);

    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    let mut buckets_list: Vec<String> = Vec::new();
    for bucket in buckets {
        buckets_list.push(bucket.name().unwrap_or_default().to_string());
    }
    println!("Found {} buckets.", num_buckets);

    Ok(buckets_list)
}