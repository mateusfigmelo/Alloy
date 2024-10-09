pub(crate) mod builder {
    use alloy::{
        network::{EthereumWallet, TransactionBuilder},
        node_bindings::Anvil,
        primitives::U256,
        providers::{Provider, ProviderBuilder},
        rpc::types::TransactionRequest,
        signers::local::PrivateKeySigner,
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing provider builder*********");
        let anvil = Anvil::new().block_time(1).try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer.clone());

        let alice = signer.address();
        let bob = anvil.addresses()[1];

        let rpc_url = anvil.endpoint_url();
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(rpc_url);
        let tx = TransactionRequest::default()
            .with_to(bob)
            .with_value(U256::from(100));

        let pending_tx = provider.send_transaction(tx).await?;
        println!("Pending transactions... {}", pending_tx.tx_hash());

        let receipt = pending_tx.get_receipt().await?;
        println!(
            "Transaction included in block {}",
            receipt.block_number.expect("Failed to get block number")
        );

        assert_eq!(receipt.from, alice);
        assert_eq!(receipt.to, Some(bob));

        Ok(())
    }
}

pub(crate) mod builtin {
    use alloy::{
        node_bindings::Anvil,
        providers::{Provider, ProviderBuilder},
    };
    use eyre::Result;
    use futures_util::StreamExt;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing provider builtin*********");
        let anvil = Anvil::new().block_time(1).try_spawn()?;

        let http_rpc_url = anvil.endpoint();
        let http_provider = ProviderBuilder::new().on_builtin(&http_rpc_url).await?;
        let block_number = http_provider.get_block_number().await?;
        println!("Latest block number: {block_number:?}");

        let ws_rpc_url = anvil.ws_endpoint();
        let ws_provider = ProviderBuilder::new().on_builtin(&ws_rpc_url).await?;
        let sub = ws_provider.subscribe_blocks().await?;
        let mut stream = sub.into_stream().take(2);
        println!("Awaiting blocks...");

        let handle = tokio::spawn(async move {
            while let Some(block) = stream.next().await {
                println!("{}", block.header.number);
            }
        });
        handle.await?;

        let ipc_path = "tmp/reth.ipc";
        let ipc_provider = ProviderBuilder::new().on_builtin(ipc_path).await?;
        let _block_number = ipc_provider.get_block_number().await?;

        Ok(())
    }
}

pub(crate) mod http {
    use alloy::providers::{Provider, ProviderBuilder};
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing provider http*********");
        let rpc_url = "https://eth.merkle.io".parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);

        let latest_block = provider.get_block_number().await?;
        println!("Latest block number: {}", latest_block);

        Ok(())
    }
}

pub(crate) mod ws {
    use alloy::providers::{Provider, ProviderBuilder, WsConnect};
    use eyre::Result;
    use futures_util::StreamExt;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing provider ws*********");
        let rpc_url = "wss://eth-mainnet.g.alchemy.com/v2/6s8PYp-UrorBla48RX2NcAdtTiE2WmtN";
        let ws = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        let sub = provider.subscribe_blocks().await?;

        let mut stream = sub.into_stream().take(4);
        println!("Awaiting blocks...");

        let handle = tokio::spawn(async move {
            while let Some(block) = stream.next().await {
                println!("Latest block number: {}", block.header.number);
            }
        });

        handle.await?;
        Ok(())
    }
}
