use aptos_sdk::move_types::account_address::AccountAddress;
use aptos_sdk::types::LocalAccount;
use dotenv::dotenv;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Url};

#[derive(Debug)]
pub struct Config {
    pub aptos_account: LocalAccount,
    pub aptos_module_address: AccountAddress,
    pub madara_provider: JsonRpcClient<HttpTransport>,
}

impl Config {
    pub fn new(madara_url: Option<String>, private_key: String) -> Self {
        dotenv().ok();
        let madara_provider = JsonRpcClient::new(HttpTransport::new(
            Url::parse(madara_url.unwrap().as_str()).unwrap(),
        ));

        let aptos_account = LocalAccount::from_private_key(private_key.as_str(), 0).unwrap();
        let aptos_module_address = aptos_account.address();

        Self {
            aptos_account,
            aptos_module_address,
            madara_provider,
        }
    }
}
