use alloy::{
    network::TransactionBuilder,
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
};
use eyre::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("*********testing gas filler*********");
    let provider = ProviderBuilder::new()
        .with_gas_estimation()
        .on_anvil_with_wallet();
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default()
        .with_to(vitalik)
        .with_value(U256::from(100))
        .with_nonce(0)
        .with_chain_id(provider.get_chain_id().await?);
    let builder = provider.send_transaction(tx.clone()).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx = provider
        .get_transaction_by_hash(node_hash)
        .await?
        .expect("Pending transaction not found");
    assert_eq!(pending_tx.nonce, 0);
    println!("Transaction sent with nonce: {}", pending_tx.nonce);

    let tx = tx.with_nonce(1);
    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();
    let pending_tx = provider
        .get_transaction_by_hash(node_hash)
        .await?
        .expect("Pending transaction not found");
    assert_eq!(pending_tx.nonce, 1);
    println!("Transaction sent with nonce: {}", pending_tx.nonce);
    Ok(())
}
