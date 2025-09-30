#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod erc20;
use crate::erc20::{Erc20, Erc20Params}
use alloc::vec::Vec;
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

struct StylusERC20Params;

impl Erc20Params for StylusERC20Params {
    const NAME: &'static str = "ERC20 Example";
    const SYMBOL: &'static str = "EE";
    const DECIMALS: u8 = 18;
}

sol_storage! {
    #[entrypoint]
    struct StylusERC20 {
        #[borrow]
        Erc20<StylusERC20Params> erc20;
    }
}

#[external]
#[inherit(Erc20<StylusERC20Params>)]
impl StylusERC20 {
    pub fn mint(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.erc20.mint(msg::sender(), amount);
        Ok(())
    }
}
