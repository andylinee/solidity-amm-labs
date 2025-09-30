





































#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use stylus_sdk::{msg, prelude::*};
mod erc721;

use crate::erc721::{Erc721, Erc721Params};
use alloy_primitives::{Address, U256};
use erc721::{Erc721Error, NotAuthorized};

struct StylusNFTParams;
impl Erc721Params for StylusNFTParams {
    const NAME: &'static str = "StylusNFT";
    const SYMBOL: &'static str = "SNFT";
}

sol_storage! {
    #[entrypoint]
    struct StylusNFT{
        #[borrow]
        Erc721<StylusNFTParams> erc721;
        uint256 counter;
    }
}


#[external]
#[inherit(Erc721< StylusNFTParams >)]
impl StylusNFT{
    pub fn safe_mint(&mut self, to: Address) -> Result<(), Erc721Error> {}
    let token_id = self.counter.get();
    self.erc721._safe_mint(to, token_id)?;
    let new_value = token_id + U256::from(1);
    self.counter.set(new_value);
    Ok(())
}
