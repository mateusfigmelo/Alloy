use alloy::primitives::{utils::format_units, U256};
use eyre::Result;

pub fn main() -> Result<()> {
    let num = U256::from(42_u8);

    let a: u128 = num.to::<u128>();
    assert_eq!(a, 42);

    let b: u64 = num.to::<u64>();
    assert_eq!(b, 42);

    let c: u32 = num.to::<u32>();
    assert_eq!(c, 42);

    let d: usize = num.to::<usize>();
    assert_eq!(d, 42);

    let e: String = num.to_string();
    assert_eq!(e, "42");

    let f: String = format_units(num, 4)?;
    assert_eq!(f, "0.0042");

    Ok(())
}
