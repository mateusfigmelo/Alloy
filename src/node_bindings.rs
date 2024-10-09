pub(crate) mod deploy_contract_on_local_anvil_instance {
    use alloy::{primitives::U256, providers::ProviderBuilder, sol};
    use eyre::Result;

    sol! {
        #[allow(missing_docs)]
        #[sol(rpc,bytecode="6080806040523460135760df908160198239f35b600080fdfe6080806040526004361015601257600080fd5b60003560e01c9081633fb5c1cb1460925781638381f58a146079575063d09de08a14603c57600080fd5b3460745760003660031901126074576000546000198114605e57600101600055005b634e487b7160e01b600052601160045260246000fd5b600080fd5b3460745760003660031901126074576020906000548152f35b34607457602036600319011260745760043560005500fea2646970667358221220e978270883b7baed10810c4079c941512e93a7ba1cd1108c781d4bc738d9090564736f6c634300081a0033")]
        contract Counter{
            uint256 public number;
            function setNumber(uint256 newNumber) public{
                number=newNumber;
            }
            function increment() public{
                number++;
            }
        }
    }

    #[tokio::main]
    pub async fn main() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_wallet();
        let contract = Counter::deploy(&provider).await?;
        println!("Contract deployed at address: {}", contract.address());

        let builder = contract.setNumber(U256::from(42));
        let tx_hash = builder.send().await?.watch().await?;
        println!("Set number to 42:{tx_hash}");

        let builder = contract.increment();
        let tx_hash = builder.send().await?.watch().await?;
        println!("Incremented number:{tx_hash}");

        let builder = contract.number();
        let number = builder.call().await?.number.to_string();
        println!("Current number:{}", number);

        Ok(())
    }
}

pub(crate) mod anvil_fork_instance {
    use alloy::{
        node_bindings::Anvil,
        providers::{ext::AnvilApi, ProviderBuilder},
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        let rpc_url = "https://eth.merkle.io";
        let anvil = Anvil::new().fork(rpc_url).try_spawn()?;
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_http(anvil.endpoint_url());

        let info = provider.anvil_node_info().await?;
        println!("Anvil node info:{:?}", info);
        assert_eq!(info.environment.chain_id, 1);
        assert_eq!(info.fork_config.fork_url, Some(rpc_url.to_string()));

        Ok(())
    }
}

pub(crate) mod anvil_fork_provider {
    use alloy::providers::{ext::AnvilApi, ProviderBuilder};
    use eyre::Result;
    #[tokio::main]
    pub async fn main() -> Result<()> {
        let rpc_url = "https://eth.merkle.io";
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_config(|anvil| anvil.fork(rpc_url));
        let info = provider.anvil_node_info().await?;
        println!("Anvil node info:{:?}", info);
        assert_eq!(info.environment.chain_id, 1);
        assert_eq!(info.fork_config.fork_url, Some(rpc_url.to_string()));
        Ok(())
    }
}

pub(crate) mod anvil_local_instance {
    use alloy::{
        node_bindings::Anvil,
        providers::{ext::AnvilApi, ProviderBuilder},
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        let anvil = Anvil::new().block_time(1).chain_id(1337).try_spawn()?;
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_http(anvil.endpoint_url());
        let info = provider.anvil_node_info().await?;
        println!("Anvil node info:{:?}", info);
        assert_eq!(info.environment.chain_id, 1337);
        assert_eq!(info.fork_config.fork_url, None);
        Ok(())
    }
}

pub(crate) mod anvil_local_provider {
    use alloy::providers::{ext::AnvilApi, ProviderBuilder};
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_anvil_with_config(|anvil| anvil.block_time(1).chain_id(1337));
        let info = provider.anvil_node_info().await?;
        println!("Anvil node info:{:?}", info);
        assert_eq!(info.environment.chain_id, 1337);
        assert_eq!(info.fork_config.fork_url, None);
        Ok(())
    }
}

pub(crate) mod geth_local_instance {
    use alloy::{
        node_bindings::Geth,
        providers::{Provider, ProviderBuilder},
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        let geth = Geth::new()
            .chain_id(1337)
            .port(8545_u16)
            .authrpc_port(8551)
            .spawn();
        let provider = ProviderBuilder::new().on_http(geth.endpoint().parse()?);
        let chain_id = provider.get_chain_id().await?;

        println!(
            "Geth running at;{} with chain id: {chain_id}",
            geth.endpoint()
        );

        assert_eq!(chain_id, 1337);
        assert_eq!(geth.port(), 8545);
        assert_eq!(geth.auth_port(), Some(8551));
        assert_eq!(geth.p2p_port(), None);

        Ok(())
    }
}