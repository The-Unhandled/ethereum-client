use ethers::types::U256;
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct Price {
    pub raw: u128,
}

impl Price {
    pub fn new(raw: u128) -> Self {
        Self { raw }
    }

    /// Convert the raw price (assumed to have 8 decimals) to USD as f64.
    pub fn to_usd(&self) -> f64 {
        self.raw as f64 / 1e8
    }
}

// Implement conversion from U256 to Price
impl From<U256> for Price {
    fn from(value: U256) -> Self {
        let raw = u128::try_from(value).expect("Price too large for u128");
        Price::new(raw)
    }
}
