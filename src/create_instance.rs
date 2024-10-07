use alloy::primitives::{utils::parse_units, U256};

use eyre::Result;
use std::str::FromStr;

pub fn main() -> Result<()> {
    let a = U256::from_str("42")?;
    assert_eq!(a, U256::from(42));

    let amount = "42";
    let units = 4;
    let b = parse_units(amount, units)?;
    assert_eq!(b.to_string(), "420000");

    let c = U256::from(42_u8);
    assert_eq!(c.to_string(), "42");

    Ok(())
}
