// Introduction to Stylus Rust SDK

// 1. Stylus Rust SDK Smart Contract
use stylus_sdk::{alloy_primitives::U256, prelude::*};

// Generate Solidity-equivalent, Rust structs backed by storage.
sol_storage! {
  #[entrypoint]
  pub struct Counter {
    uint256 number;
  }
}

#[external]
impl Counter {
  // Gets the number value from storage.
  pub fn number(&self) -> Result<U256, Vec<u8>> {
    Ok(self.number.get())
  }

  // Sets a number in storage to a user-specified value.
  pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
    self.number.set(new_number);
    Ok(())
  }
}
