use alloy::primitives::{utils::format_units, U256};
use eyre::Result;
use std::ops::{Div, Mul};

pub fn main() -> Result<()> {
    let a = U256::from(10);
    let b = U256::from(2);

    let sum = a + b;
    assert_eq!(sum, U256::from(12));

    let difference = a - b;
    assert_eq!(difference, U256::from(8));

    let product = a * b;
    assert_eq!(product, U256::from(20));

    let quotient = a / b;
    assert_eq!(quotient, U256::from(5));

    let remainder = a % b;
    assert_eq!(remainder, U256::from(0));

    let power = a.pow(b);
    assert_eq!(power, U256::from(100));

    let eth1 = U256::from(10_000_000_000_000_000_000_u128);
    let eth2 = U256::from(20_000_000_000_000_000_000_u128);
    let base = U256::from(10).pow(U256::from(18));
    let mul = eth1.mul(eth2).div(base);
    let s: String = format_units(mul, "ether")?;
    assert_eq!(s, "200.000000000000000000");

    Ok(())
}
