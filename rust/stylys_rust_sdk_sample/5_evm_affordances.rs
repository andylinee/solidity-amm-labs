// EVM affordances in Stylus Rust SDK
// 1. Events
// 1.1 Real Use Case
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

pub fn transfer_impl(
    &mut self,
    from: Address,
    to: Address,
    value: U256,
) -> Result<(), Erc20Error> {
    let mut sender_balance = self.balances.setter(from);
    let old_sender_balance = sender_balance.get();
    if old_sender_balance < value {
        return Err(Erc20Error::InsufficientBalance(InsufficientBalance {
            from,
            have: old_sender_balance,
            want: value,
        }));
    }
    sender_balance.set(old_sender_balance - value);
    let mut to_balance = self.balances.setter(to);
    let new_to_balance = to_balance.get() + value;
    to_balance.set(new_to_balance);
    evm::log(Transfer { from, to, value });
    Ok(())
}


// 1.2 Documentation
// 1.2.1 Declaring Events
event Transfer(address indexed from, address indexed to, uint256 value);

// 1.2.2 Emitting Events
// Define the _transfer function to execute the transfer logic
function _transfer(
    address from,
    address to,
    uint256 amount
) external {

    _balances[from] = 10000000; // Give the transfer address some initial tokens

    _balances[from] -=  amount; // Subtract the transfer amount from the from address
    _balances[to] += amount; // Add the transfer amount to the to address

    // Emit the event
    emit Transfer(from, to, amount);
}


// 1.3 Example
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

fn foo() {
   ...
   evm::log(Transfer {
      from: Address::ZERO,
      to: address,
      value,
   });
}

// 1.4 Quest
pub fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Erc20Error> {
    self.allowances.setter(msg::sender()).insert(spender, value);
    
evm::log(Approval {
        owner: msg::sender(),
        spender,
        value,
    });

    Ok(true)
}


// 2. EVM affordances
// 2.1 Real Use Case
pub fn domain_separator(&mut self) -> Result<B256> {
    if block::chainid() == T::INITIAL_CHAIN_ID {
        Ok(T::INITIAL_DOMAIN_SEPARATOR)
    } else {
        Ok(ERC20::<T>::compute_domain_separator()?)
    }
} 


// 2.2 Documentation
use stylus_sdk::{block, contract, crypto, evm, msg, tx};

let number = block::number();

let balance = contract::balance();

let preimage = address!("361594F5429D23ECE0A88E4fBE529E1c49D524d8");
let hash = crypto::keccak(&preimage);

let gas = evm::gas_left();

let call_value = msg::value();

let gas_price = tx::gas_price();


// 2.3 Example
use stylus_sdk::{call, evm, msg, prelude::*};

pub fn approve(&mut self, spender: Address, amount: U256) -> Result<bool> {
    self.allowance.setter(msg::sender()).insert(spender, amount);

    evm::log(Approval {
        owner: msg::sender(),
        spender,
        amount,
    });

    Ok(true)
}


// 2.4 Quest
impl Proxy {
    pub fn only_owner(&mut self) -> Result<(), Vec<u8>> {
        let owner = self.meta_information.owner.get();
        if owner != msg::sender() {
            return Err(format!("Invalid").into());
        }
        Ok(())

    }
}
