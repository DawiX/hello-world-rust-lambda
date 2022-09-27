use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{
//    json, 
    Value
};

mod s3;
mod aws_config;
mod model;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<model::Response, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");
    println!("Got first name: {}",first_name );

    // if you want to use this, import: use std::env; and this way env var will be required by Rust
    //let v = env::var("SOMEVAR").expect("$SOMEVAR is not set");
    //println!(" SOMEVAR = {}", v);

    let config = aws_config::generate_config().await;
    let buckets = s3::list_buckets(config).await;

    let lambda_response = model::Response {
        message: format!("Hello, {}!", first_name),
        buckets: buckets?,
    };

    // if I would want to serialize JSON struct for whatever reason
    //let response_json = serde_json::to_string(&lambda_response)?;

    Ok(lambda_response)
}
