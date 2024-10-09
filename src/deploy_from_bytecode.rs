use alloy::{
    hex,
    network::TransactionBuilder,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
};
use eyre::Result;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract Counter{
        uint256 public number;
        function setNumber(uint256 _number) public{
            number = _number;
        }
        function increment() public{
            number++;
        }
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("*********testing deploy contract from bytecode*********");
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .on_anvil_with_wallet();
    let bytecode=hex::decode(
        "6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033"
    )?;

    let tx = TransactionRequest::default().with_deploy_code(bytecode);
    let receipt = provider.send_transaction(tx).await?.get_receipt().await?;

    let contract_address = receipt.contract_address.expect("Failed to deploy contract");
    let contract = Counter::new(contract_address, &provider);
    println!("Contract deployed at: {}", contract.address());

    let builder = contract.setNumber(U256::from(42));
    let tx_hash = builder.send().await?.watch().await?;
    println!("Set number to 42:{tx_hash}");

    let builder = contract.increment();
    let tx_hash = builder.send().await?.watch().await?;
    println!("Incremented number:{tx_hash}");

    let builder = contract.number();
    let number: String = builder.call().await?.number.to_string();
    println!("Current number: {}", number);

    Ok(())
}
