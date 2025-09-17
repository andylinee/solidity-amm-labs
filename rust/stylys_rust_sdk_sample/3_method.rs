// Methos in Stylus Rust SDK

// 1. Visibility of Method (external)
// 1.1 Use Case
sol_storage! {
    pub struct Erc20<T> {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
        PhantomData<T> phantom;
    }
}

impl<T: Erc20Params> Erc20<T> {
		
    pub fn transfer_impl(
	    &mut self, from: Address, to: Address, value: U256,
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
}

#[external]
impl<T: Erc20Params> Erc20<T> {
    pub fn balance_of(&self, address: Address) -> Result<U256, Erc20Error> {
        Ok(self.balances.get(address))
    }

    pub fn mint(&mut self, address: Address, value: U256) {
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);
        self.total_supply.set(self.total_supply.get() + value);
    }
		
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self.transfer_impl(msg::sender(), to, value)?;
        Ok(true)
    }
}


// 1.2 Documentation
// innerMethod: internal function
function innerMethod() internal {
    number = number - 1;
}

// Functions within the contract can call internal functions
function outerMethod() external {
    innerMethod();
}


// 1.3 Example
#[external]
impl Counter {
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())
    }

    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
        Ok(())
    }

    pub fn increment(&mut self) -> Result<(), Vec<u8>> {
        let number = self.number.get();
        self.set_number(number + U256::from(1))
    }
}

impl Counter {
    pub fn other_internal_op(&mut self) -> Result<(), Vec<u8>> {
        ...
    }
}


// 1.4 Quest
#[external]

impl<T: Erc20Params> Erc20<T> {
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Erc20Error> {
        self.transfer_impl(msg::sender(), to, value)?;
        Ok(true)
    }

    pub fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Erc20Error> {
        self.allowances.setter(msg::sender()).insert(spender, value);
        evm::log(Approval {
            owner: msg::sender(),
            spender,
            value,
        });
        Ok(true)
    }
}


// 2. Contract entry point
// 2.1 Documentation
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}


// 2.2 Example
sol_storage! {
   #[entrypoint]
    pub struct Contract {
        address owner;
        bool active;
        SubStruct sub_struct,
    }

    pub struct SubStruct {
        // ...
    }
}

// 2.3 Quest
sol_storage! {
    // Makes SampleNFT the entrypoint
    #[entrypoint]

    pub struct SampleNFT {
        #[borrow] // Allows erc721 to access SampleNFT's storage and make calls
        ERC721<SampleParams> erc721;
        uint256 total_supply;
    }
}


// 3. Send and Receive ETH
// 3.1 Use Case
pub fn withdraw(&mut self, amount: U256) -> Result<(), Vec<u8>> {
    self.beforeWithdraw(amount);

    let supply = self.totalSupply.get();
    let shares = if supply == U256::ZERO {
        amount
    } else {
        amount.checked_mul(supply).ok_or("Overflow")?.checked_div(self.totalAssets()?).ok_or("Divide by zero")?
    };

    self.erc20.burn(msg::sender(), amount)?;
    call::transfer_eth(msg::sender(), amount)
}


// 3.2 Documentation
pragma solidity ^0.8.0;

contract Fundraiser {
    function donate() external payable {
        // Ether is received and stored in the contract's balance
        // You can perform any other actions with the Ether received here - for example, sending it to some other address etc.
    }
}


#[external]
impl Contract {
    #[payable]
    pub fn credit(&mut self) -> Result<(), Vec<u8>> {
        self.erc20.add_balance(msg::sender(), msg::value())
    }
}


// 3.3 Example
transfer_eth(recipient, value)?;                 // these two are equivalent

call(Call::new().value(value), recipient, &[])?; // these two are equivalent


// 3.4 Quest
#[external]
impl Fundraiser {
    #[payable]

    pub fn donate(&mut self) -> Result<(), Vec<u8>> {
        // ...
07
    }
}


// 4. Inheritance of Contract
// 4.1 Use Case
sol_storage! {
    #[entrypoint]
    struct Weth {
        #[borrow] // Allows erc20 to access Weth's storage and make calls
        Erc20<WethParams> erc20;
    }
}

#[external]
#[inherit(Erc20<WethParams>)]
impl Weth {
    #[payable]
    pub fn deposit(&mut self) -> Result<(), Vec<u8>> {
        self.erc20.mint(msg::sender(), msg::value());
        Ok(())
    }

    pub fn withdraw(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        self.erc20.burn(msg::sender(), amount)?;

        call::transfer_eth(msg::sender(), amount)
    }
}


// 4.2 Documentation
// 4.2.1
#[external]
#[inherit(Erc20)]
impl Token {
    pub fn mint(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        ...
    }
}

#[external]
impl Erc20 {
    pub fn balance_of() -> Result<U256> {
        ...
    }
}

// 4.2.2
sol_storage! {
    #[entrypoint]
    pub struct Token {
        #[borrow]
        Erc20 erc20;
        ...
    }

    pub struct Erc20 {
        ...
    }
}


// 4.3 Example
sol_storage! {
    #[entrypoint]
    struct Vault {
        address asset;
        uint totalSupply;

        #[borrow]
        Erc20<VaultParams> erc20;
    }
}

#[external]
#[inherit(Erc20<VaultParams>)]
impl Vault {

    #[payable]
    pub fn deposit(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        // ...
        self.erc20.mint(msg::sender(), amount);

        Ok(())
    }
}


// 4.4 Quest
sol_storage! {
    #[entrypoint]
    struct Child {
        address asset;
        uint totalSupply;

        #[borrow]

        Parent parent;
    }
}

#[external]
#[inherit(Parent)]
impl Child {

    pub fn child_method(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        // ...
        self.parent.parent_method();

        Ok(())
    }
}
