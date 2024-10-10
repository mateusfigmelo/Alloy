pub(crate) mod decode_input {
    use alloy::{primitives::hex, sol, sol_types::SolCall};
    use eyre::Result;

    sol!(
        #[allow(missing_docs)]
        function swapExactTokensForTokens(
            uint256 amountIn,
            uint256 amountOutMin,
            address[] calldata path,
            address to,
            uint256 deadline
        ) external returns(uint256[] memory amounts);
    );

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing decode input*********");
        println!("Decoding https://etherscan.io/tx/0xd1b449d8b1552156957309bffb988924569de34fbf21b51e7af31070cc80fe9a");
        let input="0x38ed173900000000000000000000000000000000000000000001a717cc0a3e4f84c00000000000000000000000000000000000000000000000000000000000000283568400000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000201f129111c60401630932d9f9811bd5b5fff34e000000000000000000000000000000000000000000000000000000006227723d000000000000000000000000000000000000000000000000000000000000000200000000000000000000000095ad61b0a150d79219dcf64e1e6cc01f0b64c4ce000000000000000000000000dac17f958d2ee523a2206206994597c13d831ec7";
        let input = hex::decode(input)?;
        let decoded = swapExactTokensForTokensCall::abi_decode(&input, false);

        match decoded {
            Ok(decoded) => {
                let path = decoded.path;
                println!(
                    "Swap {} of token {} to {} of token {}",
                    decoded.amountIn,
                    path.first().expect("Path is empty"),
                    decoded.amountOutMin,
                    path.last().expect("Path is empty")
                );
            }
            Err(e) => {
                println!("Error decoding input: {e:?}");
            }
        }

        Ok(())
    }
}

pub(crate) mod encode_decode_eip1559 {
    use alloy::{
        consensus::{SignableTransaction, TxEip1559},
        eips::eip2930::AccessList,
        primitives::{address, b256, hex, Signature, TxKind, U256},
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing encode decode eip1559*********");
        // EIP1559 transaction: <https://etherscan.io/tx/0x0ec0b6a2df4d87424e5f6ad2a654e27aaeb7dac20ae9e8385cc09087ad532ee0>

        let tx_hash = b256!("0ec0b6a2df4d87424e5f6ad2a654e27aaeb7dac20ae9e8385cc09087ad532ee0");
        let signer = address!("DD6B8b3dC6B7AD97db52F08a275FF4483e024CEa");
        let tx=TxEip1559{
            chain_id:1,
            nonce:0x42,
            gas_limit:44386,
            to:TxKind::Call(address!("6069a6c32cf691f5982febae4faf8a6f3ab2f0f6")),
            value:U256::from(0_u64),
            input:hex!("a22cb4650000000000000000000000005eee75727d804a2b13038928d36f8b188945a57a0000000000000000000000000000000000000000000000000000000000000000").into(),
            max_fee_per_gas:0x4a817c800,
            max_priority_fee_per_gas:0x3b9aca00,
            access_list:AccessList::default(),
        };
        let signature = Signature::from_scalars_and_parity(
            b256!("840cfc572845f5786e702984c2a582528cad4b49b2a10b9db1be7fca90058565"),
            b256!("25e7109ceb98168d95b09b18bbf6b685130e0562f233877d492b94eee0c5b6d1"),
            false,
        )?;
        let signed_tx = tx.into_signed(signature);
        assert_eq!(*signed_tx.hash(), tx_hash);
        let recovered_signer = signed_tx.recover_signer()?;
        assert_eq!(recovered_signer, signer);

        Ok(())
    }
}

pub(crate) mod gas_price_usd {
    use alloy::{
        network::TransactionBuilder,
        primitives::{address, utils::format_units, Address, Bytes, U256},
        providers::{Provider, ProviderBuilder},
        rpc::types::request::TransactionRequest,
        sol,
        sol_types::SolCall,
    };
    use eyre::Result;
    use std::str::FromStr;

    const ETH_USD_FEED: Address = address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");
    const ETH_USD_FEED_DECIMALS: u8 = 8;
    const ETH_DECIMALS: u32 = 18;

    sol! {
        #[allow(missing_docs)]
        function latestAnswer() external view returns(int256);
    }

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing gas price usd*********");
        let rpc_url = "https://eth.merkle.io";
        let provider = ProviderBuilder::new().on_anvil_with_config(|anvil| anvil.fork(rpc_url));

        let call = latestAnswerCall {}.abi_encode();
        let input = Bytes::from(call);

        let tx = TransactionRequest::default()
            .with_to(ETH_USD_FEED)
            .with_input(input);
        let response = provider.call(&tx).await?;
        let result = U256::from_str(&response.to_string())?;

        let wei_per_gas = provider.get_gas_price().await?;

        let gwei = format_units(wei_per_gas, "gwei")?.parse::<f64>()?;
        let usd = get_usd_value(wei_per_gas, result)?;

        println!("Gas price in Gwei:{gwei}");
        println!("Gas price in USD:{usd}");

        Ok(())
    }

    fn get_usd_value(amount: u128, price_usd: U256) -> Result<f64> {
        let base = U256::from(10).pow(U256::from(ETH_DECIMALS));
        let value = U256::from(amount) * price_usd / base;
        let formatted = format_units(value, ETH_USD_FEED_DECIMALS)?.parse::<f64>()?;

        Ok(formatted)
    }
}

pub(crate) mod send_raw_transaction {
    use alloy::{
        network::TransactionBuilder,
        primitives::U256,
        providers::{Provider, ProviderBuilder, WalletProvider},
        rpc::types::request::TransactionRequest,
    };
    use eyre::Result;

    #[tokio::main]
    pub async fn main() -> Result<()> {
        println!("*********testing send raw transaction*********");

        let provider = ProviderBuilder::new().on_anvil_with_wallet();

        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];

        let tx = TransactionRequest::default()
            .with_to(bob)
            .with_nonce(0)
            .with_chain_id(provider.get_chain_id().await?)
            .with_value(U256::from(100))
            .with_gas_limit(21000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000);

        let tx_envelope = tx.build(&provider.wallet()).await?;
        let receipt = provider
            .send_tx_envelope(tx_envelope)
            .await?
            .get_receipt()
            .await?;

        println!("Transaction hash: {}",receipt.transaction_hash);
        assert_eq!(receipt.from, alice);
        assert_eq!(receipt.to, Some(bob));

        Ok(())
    }
}
