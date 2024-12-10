use crate::aptos::emit_event_for_block;
use crate::config::Config;
use crate::madara::fetch_block;
use anyhow::Result;
use aptos_sdk::rest_client::Client;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::LocalAccount;
use aptos_testcontainer::test_utils::aptos_container_test_utils::{lazy_aptos_container, run};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use url::Url;

pub async fn run_task(config: Config) -> Result<()> {
    run(1, |accounts| {
        Box::pin(async move {
            println!("[👌] Start creating Aptos container!");

            let container = lazy_aptos_container().await.unwrap();
            let module_account_private_key = accounts.first().unwrap().to_string();
            let aptos_account =
                LocalAccount::from_private_key(module_account_private_key.clone().as_str(), 0)
                    .unwrap();
            let module_address = aptos_account.address();

            let mut named_address = HashMap::new();
            named_address.insert("viktor".to_string(), module_address.clone().to_string());

            println!("[🍻] Start deploying contract!");

            container
                .upload_contract(
                    "./contract",
                    module_account_private_key.clone().as_str(),
                    &named_address,
                    None,
                    false,
                )
                .await
                .unwrap();

            println!(
                "[👊] Contract deployed at address {} on network {}.",
                module_address.clone(),
                container.get_node_url()
            );

            let interval = Duration::from_secs(10);
            let mut next_time = Instant::now() + interval;
            let mut fetched_block = 0u64;

            let madara_provider = config.madara_provider;
            let aptos_chain_id = ChainId::new(container.get_chain_id());
            let aptos_client = Client::new(Url::parse(container.get_node_url().as_str()).unwrap());

            println!("[🍬] Start fetching block!");

            loop {
                let block_number = fetch_block(&madara_provider).await.unwrap();
                println!("[🧊] Madara block number: {:?}", block_number);
                while block_number > fetched_block {
                    println!("[🕐] Fetching block: {:?}", fetched_block);
                    emit_event_for_block(
                        &aptos_account,
                        aptos_chain_id,
                        &aptos_client,
                        fetched_block,
                    )
                    .await;
                    fetched_block += 1;
                }
                sleep(next_time - Instant::now());
                next_time += interval;
            }
        })
    })
    .await
}
