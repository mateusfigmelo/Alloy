use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    node_bindings::Anvil,
    primitives::{address, b256, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    let anvil = Anvil::new().try_spawn()?;

    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::from(signer);

    let rpc_url = anvil.endpoint_url();
    let provider = ProviderBuilder::new().wallet(wallet).on_http(rpc_url);
    let vitalik = address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    let tx = TransactionRequest::default()
        .with_to(vitalik)
        .with_value(U256::from(100))
        .with_nonce(0)
        .with_gas_limit(21_000)
        .with_gas_price(20_000_000_000);

    let builder = provider.send_transaction(tx).await?;
    let node_hash = *builder.tx_hash();
    println!(
        "Node hash matches expected hash:{}",
        node_hash == b256!("eb56033eab0279c6e9b685a5ec55ea0ff8d06056b62b7f36974898d4fbb57e64")
    );

    let pending_tx = builder.register().await?;
    println!(
        "Pending transaction hash matched node hash:{}",
        *pending_tx.tx_hash() == node_hash
    );

    let tx_hash = pending_tx.await?;
    assert_eq!(tx_hash, node_hash);
    println!(
        "Transaction hash matches node hash:{}",
        tx_hash == node_hash
    );

    let receipt = provider
        .get_transaction_receipt(tx_hash)
        .await?
        .expect("Transaction receipt not found");
    let receipt_hash = receipt.transaction_hash;
    assert_eq!(receipt_hash, node_hash);
    println!(
        "Receipt hash matches node hash:{}",
        receipt_hash == node_hash
    );

    Ok(())
}
