use anyhow::Result;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};

pub async fn fetch_block(provider: &JsonRpcClient<HttpTransport>) -> Result<u64> {
    Ok(provider.block_number().await?)
}
