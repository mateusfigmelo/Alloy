pub(crate) mod contract_storage {
    use alloy::{
        primitives::{address, U256},
        providers::{Provider, ProviderBuilder},
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing contract storage*********");

        let rpc_url = "https://eth.merkle.io".parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);
        //Get storage slot  0 from the UniswapV3 USDC-ETH pool on Ethereum mainnet
        let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
        let storage_slot = U256::from(0);
        let storage = provider.get_storage_at(pool_address, storage_slot).await?;

        println!("Storage slot 0 of UniswapV3 USDC-ETH pool: {:?}", storage);
        Ok(())
    }
}

pub(crate) mod query_logs {
    use alloy::{
        primitives::{address, b256},
        providers::{Provider, ProviderBuilder},
        rpc::types::Filter,
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing query logs*********");
        let rpc_url = "https://eth.merkle.io".parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);

        let latest_block = provider.get_block_number().await?;
        let filter = Filter::new().from_block(latest_block);
        let logs = provider.get_logs(&filter).await?;
        for log in logs {
            println!("Log: {:?}", log);
        }

        let transfer_event_signature =
            b256!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef");
        let filter = Filter::new()
            .event_signature(transfer_event_signature)
            .from_block(latest_block);

        let logs = provider.get_logs(&filter).await?;
        for log in logs {
            println!("Transfer event: {:?}", log);
        }

        let uniswap_token_address = address!("1f9840a85d5af5bf1d1762f925bdaddc4201f984");
        let filter = Filter::new()
            .address(uniswap_token_address)
            .from_block(latest_block);
        let logs = provider.get_logs(&filter).await?;
        for log in logs {
            println!("Uniswap token log: {:?}", log);
        }
        Ok(())
    }
}
