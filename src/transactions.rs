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
