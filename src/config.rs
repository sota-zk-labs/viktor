use dotenv::dotenv;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Url};

#[derive(Debug)]
pub struct Config {
    pub madara_provider: Option<JsonRpcClient<HttpTransport>>,
    pub running_mode: Mode,
}

#[derive(Debug)]
pub enum Mode {
    Run,
    Deploy,
}

impl Config {
    pub fn new(madara_url: Option<String>, running_mode: Mode) -> Self {
        dotenv().ok();
        let madara_provider: Option<JsonRpcClient<HttpTransport>> = madara_url.map(|url| {
            JsonRpcClient::new(HttpTransport::new(
                Url::parse(url.as_str()).unwrap(),
            ))
        });

        Self { madara_provider, running_mode }
    }
}
