// Storage in Stylus Rust SDK

// 1. Storage (sol_Storage)
// 1.1 Use Case
sol_storage! {
    pub struct Erc20<T> {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
        PhantomData<T> phantom;
    }
}

// 1.2 Documentation
// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.6.0 <0.9.0;

contract SimpleStorage {
    address owner;
    bool active;
    SubStruct sub_struct;

    struct SubStruct {
        // other solidity fields, such as
        mapping(address => uint) balances;
        Delegate[] delegates;
    }
}

// 1.3 Example
sol_storage! {
    pub struct Contract {
        address owner;                      
        bool active;                        
        SubStruct sub_struct,
    }

    pub struct SubStruct {
        // other solidity fields, such as
        mapping(address => uint) balances;  
        Delegate delegates[];               
    }
}

// 1.4 Quest
sol_storage! {
    pub struct StorageDemo {
        address owner;    
        bool active;
        string name,
    }
}

// 2. Reading and Writing Storage (get & set)
// 2.1 Use Case
sol_storage! {
    pub struct Erc20<T> {
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        uint256 total_supply;
        PhantomData<T> phantom;
    }
}

impl<T: Erc20Params> Erc20<T> {
    pub fn mint(&mut self, address: Address, value: U256) {
        let mut balance = self.balances.setter(address);
        let new_balance = balance.get() + value;
        balance.set(new_balance);
        self.total_supply.set(self.total_supply.get() + value);
    }
}


// 2.2 Documentation
#[solidity_storage]
pub struct Contract {
    owner: StorageAddress,
}

impl Contract {
    /// Gets the owner from storage.
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.owner.get())
    }

    /// Updates the owner in storage
    pub fn set_owner(&mut self, new_owner: Address) -> Result<(), Vec<u8>> {
        if msg::sender() == self.owner()? {  // we'll discuss msg::sender later
            self.owner.set(new_owner);
        }
        Ok(())
    }
}


// 2.3 Example
sol_storage! {
    pub struct SubStruct {
        mapping(address => uint) balances;  
        Delegate delegates[];              
    }
}

impl SubStruct {
    pub fn add_delegate(&mut self, delegate: Delegate) -> Result<(), Vec<u8>> {
        self.delegates.push(delegate);
    }

    pub fn track_balance(&mut self, address: Address) -> Result<U256, Vec<u8>> {
        self.balances.insert(address, address.balance());
    }
}


// 2.4 Quest
sol_storage! {
    #[entrypoint]
    pub struct Counter {
        uint256 number;
    }
}
    
#[external]
impl Counter {
    /// Gets the number from storage.
    pub fn number(&self) -> Result<U256, Vec<u8>> {
        Ok(self.number.get())

    }
    
    /// Sets a number in storage to a user-specified value.
    pub fn set_number(&mut self, new_number: U256) -> Result<(), Vec<u8>> {
        self.number.set(new_number);
    }
    
    Ok(())
}


// 3. Storage Erase
// 3.1 Use Case
sol_storage! {
    #[derive(Erase)]
    pub struct Contract {
        address owner;
        uint256[] hashes;
    }
}

impl Contract {
    fn remove(mut contract: Contract) {
        contract.owner.erase();
        contract.hashes.erase();
    }
}


// 3.2 Documentation
contract DeleteDemo{

    bool public b  = true;
    uint public i = 1;
    address public addr = msg.sender;
    bytes public varByte = "123";
    string  public str = "abc";
    enum Color{RED,GREEN,YELLOW}
    Color public color = Color.GREEN;

    function deleteAttr() public {
        delete b; // false
        delete i; // 0
        delete addr; // 0x0
        delete varByte; // 0x
        delete str; // ""
        delete color;//Color.RED
    }
}


// 3.3 Example
sol_storage! {
    #[derive(Erase)]
    pub struct Contract {
        address owner;             
        uint256[] hashes;           
    }

    pub struct NotErase {
        mapping(address => uint) balances; // can't erase a map
        mapping(uint => uint)[] roots;     // can't erase vector of maps
    }
}


// 3.4 Quest
sol_storage! {

    #[derive(Erase)]
    
    pub struct EraseContract {
        address owner;
        string name;
    }
}
