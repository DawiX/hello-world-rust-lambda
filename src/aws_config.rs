use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_types::sdk_config::SdkConfig;

pub async fn generate_config() -> SdkConfig {
    let config;
    let profile = option_env!("PROFILE").unwrap_or("").to_string();
    if profile == "" {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-west-1");
        config = aws_config::from_env().region(region_provider).load().await;
    } else {
        let credentials_provider = ProfileFileCredentialsProvider::builder()
        .profile_name(profile)
        .build();
    config = aws_config::from_env()
        .credentials_provider(credentials_provider)
        .load()
        .await;
    }
    config
}
