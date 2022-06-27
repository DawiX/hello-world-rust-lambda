# Hello World Rust Lambda

Lambda uses:

* [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
* [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)

To install cargo-lambda, simply run `python3 -m pip install -U cargo-lambda`

Lambda gets input event, passes `firstName` and returns it in the JSON response.

It also lists all the buckets in deployed account and outputs a list into the JSON response.

## Development

```bash
# cargo-edit to manage dependencies(https://github.com/killercup/cargo-edit#installation)
cargo add $DEPENDENCY # will update Cargo.toml with new dependency at its latest version
cargo upgrade # will upgrade all dependencies to latest version
cargo build # will build the binaries
cargo lambda build --release --arm64 --output-format zip # ./target/lambda/hello-rust-lambda/bootstrap.zip
```

## Deployment

Just deploy lambda in any way comfortable with the following:

* runtime = provided.al2
* handler = bootstrap
* architecture = arm64 (you can use default intel silicon, but with ARM you get same performance with 20% less cost)

If you want to go with intel based silicon, just compile without specifying the architecture

## Local

Lambda can get PROFILE as an env var and will override default AWS config with the profile, which is useful for local development.

```bash
cargo lambda watch # Starts watch task and exposes lambda on 127.0.0.1:9000
```

In other terminal session:

```bash
cargo lambda invoke --data-ascii '{"firstName": "Something"}' # will invoke lambda with specified payload
cargo lambda invoke --data-file event.json # Invokes lambda with JSON payload from file
```

Lambda in the watch task is compiled only after first invocation and then with any subsequent code change.
Use provided `start_local_env.sh` script for convenience, where the script also automatically exports all variables needed for your lambda - specifically `$PROFILE`, which is needed for local invocations to work against AWS.
