use dotenv::dotenv;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Url};

#[derive(Debug)]
pub struct Config {
    pub madara_provider: JsonRpcClient<HttpTransport>,
}

impl Config {
    pub fn new(madara_url: Option<String>) -> Self {
        dotenv().ok();
        let madara_provider = JsonRpcClient::new(HttpTransport::new(
            Url::parse(madara_url.unwrap().as_str()).unwrap(),
        ));

        Self { madara_provider }
    }
}
