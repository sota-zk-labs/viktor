use std::time::SystemTime;

use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::u256;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::rest_client::Client;
use aptos_sdk::transaction_builder::TransactionBuilder;
use aptos_sdk::types::chain_id::ChainId;
use aptos_sdk::types::transaction::{EntryFunction, SignedTransaction, TransactionPayload};
use aptos_sdk::types::LocalAccount;

pub async fn emit_event_for_block(
    account: &LocalAccount,
    chain_id: ChainId,
    client: &Client,
    block_number: u64,
) {
    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            account.address(),
            Identifier::new("starknet_validity").unwrap(),
        ),
        Identifier::new("emit_event_for_block").unwrap(),
        vec![],
        serialize_values(vec![&MoveValue::U256(u256::U256::from(block_number))].into_iter()),
    ));

    let tx = build_transaction(payload, account, chain_id);
    client
        .submit_and_wait(&tx)
        .await
        .expect("Failed to send transaction")
        .into_inner();
}

fn build_transaction(
    payload: TransactionPayload,
    sender: &LocalAccount,
    chain_id: ChainId,
) -> SignedTransaction {
    let i = sender.increment_sequence_number() + 1;
    let tx = TransactionBuilder::new(
        payload,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60,
        chain_id,
    )
    .sender(sender.address())
    .sequence_number(i)
    .max_gas_amount(30000)
    .gas_unit_price(100)
    .build();
    sender.sign_transaction(tx)
}
