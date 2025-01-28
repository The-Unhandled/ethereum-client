use ethers::types::U256;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Balance {
    pub wei: u128,
}

impl Balance {
    pub fn new(wei: u128) -> Self {
        Self { wei }
    }

    pub fn to_ether(&self) -> f64 {
        self.wei as f64 / 1e18
    }

    pub fn to_gwei(&self) -> f64 {
        self.wei as f64 / 1e9
    }

}

impl From<U256> for Balance {
    fn from(value: U256) -> Self {
        let wei = u128::try_from(value).expect("Balance too large for u128");
        Balance::new(wei)
    }
}

// Implementing the Default trait to provide a default value (zero balance)
impl Default for Balance {
    fn default() -> Self {
        Balance { wei: 0 }
    }
}
