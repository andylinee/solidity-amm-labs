// Contract Call in Stylus Rust SDK

// 1. Export of Interface
// 1.1 Real Use Case
sol_interface! {
    interface IERC721TokenReceiver {
        function onERC721Received(address operator, address from, uint256 token_id, bytes data) external returns(bytes4);
    }
}

// 1.2 Documentation
// Enable export
#![cfg_attr(not(feature = "export-abi"), no_main)]

// 1.3 Example
sol_interface! {
    interface IMath {
        function sum(uint256[] values) pure returns (string, uint256);
    }
}


// 2. Contract Call by Interface
// 2.1 Real Use Case
// import interface
sol_interface! {
    interface IMath {
        function sum(uint256[] values) pure returns (string, uint256);
    }
}

// calls the sum() method from the interface
pub fn sum_with_helper(&self, helper: IMath, values: Vec<U256>) -> Result<U256, Vec<u8>> {
    let (text, sum) = helper.sum(self, values)?;
    assert_eq!(&text, "sum");
    Ok(sum)
}


// 2.2 Documentation
pub fn do_call(&mut self, account: IService, user: Address) -> Result<String, Error> {
    account.make_payment(self, user)  // note the snake case
}


// 2.3 Example
pub fn do_call(account: IService, user: Address) -> Result<String, Error> {
    let config = Call::new()
        .gas(evm::gas_left() / 2)       // limit to half the gas left
        .value(msg::value());           // set the callvalue

    account.make_payment(config, user)
}


// 3. Low-Level Calls of Contract
// 3.1 Real Use Case
pub fn swap(
    &mut self,
    amount0_out: U256,
    amount1_out: U256,
    to: Address,
    data: Vec<u8>,
) -> Result<(), Vec<u8>> {

    // ...

    let token0 = IERC20::new(self.token0.get());
    let token1 = IERC20::new(self.token1.get());
    if amount0_out > U256::ZERO { self._safeTransfer(self.token0.get(), to, amount0_out)?; }
    if amount1_out > U256::ZERO { self._safeTransfer(self.token1.get(), to, amount1_out)?; }

    if !data.is_empty() {
        call(Call::new(), to, &data)?;
    }

    // ...

    let k = _reserve0.checked_mul(_reserve1).unwrap().checked_mul(U256::from(1000)).unwrap();
    if balance0_adjusted.checked_mul(balance1_adjusted).unwrap() < k {
        return Err("K".into());
    }

    // ...
    Ok(())
}


// 3.2 Documentation
// stylus_sdk::call::call

let return_data = call(Call::new(), target_contract, call_data)?;


// 3.3 Example
pub fn deposit(&mut self, amount: U256) -> Result<(), Vec<u8>> {
    let selector = function_selector!("transferFrom(address,address,uint256)");
    let data = [
        &selector[..],
        &msg::sender().into_array(),
        &self.recipent.get().into_array(),
        &amount.to_be_bytes::<32>(),
    ].concat();
    call(Call::new(), self.target.get(), &data);

    // ...

    Ok(())
}



// 4. Delegate Call of Contract
// 4.1 Documentation
// stylus_sdk::call::delegate_call

let return_data = delegate_call(Call::new(), target_contract, call_data)?;


// 4.2 Example
#[external]
impl Proxy {
    //...

    pub fn relay_to_implementation(&mut self, data: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        let implementation_address = self.get_implementation()?;
        let res;
        unsafe {
            res = delegate_call(self, implementation_address, &data[..])
        };

        match res {
            Ok(res) => Ok(res.into()),
            Err(e) => Err(format!("Error: {:?}", e).into()),
        }
    }
}


// 4.3 Quest
pub fn invoke_logic_contract(&mut self, spender: Address, value: U256) -> Result<bool, Erc20Error> {
    let logic_address = self.get_logic()?;
    let res;
    unsafe {
        res = delegate_call(self, logic_address, &data[..])
    };

    match res {
        Ok(res) => Ok(res.into()),
        Err(e) => Err(format!("Error: {:?}", e).into()),
    }

    Ok(true)
}
