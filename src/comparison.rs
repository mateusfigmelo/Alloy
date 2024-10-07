use alloy::primitives::U256;

pub fn main() {
    let a = U256::from(100_u32);
    let b = U256::from(100_u32);
    assert!(a == b);

    let a = U256::from(1_u32);
    let b = U256::from(100_u32);
    assert!(a < b);

    let a = U256::from(100_u32);
    let b = U256::from(100_u32);
    assert!(a <= b);

    let a = U256::from(100_u32);
    let b = U256::from(100_u32);
    assert!(a >= b);

    let a = U256::ZERO;
    assert!(a.is_zero());
}
