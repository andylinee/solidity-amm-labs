// Trait in Rust

// 1. Use Case
// solana_program::account_info::Account

pub trait Account {
    // Required method
    // Returns: lamports, data, owner, executable, rent_epoch
    fn get(&mut self) -> (&mut u64, &mut [u8], &Pubkey, bool, Epoch);
}


// 2. Example
struct WildGoose {
     color: String,
}

// WildGoose's own method
impl WildGoose {
     //Create an instance of itself
     fn new() -> Self {
         WildGoose {
             color: "gray".to_string(),
         }
     }
// Perch by the lake
     fn inhabit(&self) {
         println!("wild geese perch by the lake");
     }
}

// Swallow structure
struct Swallow {
     color: String,
}

// Swallow's own method
impl Swallow {
     fn new() -> Self {
         Swallow {
             color: "black".to_string(),
         }
     }
// Build a nest under the eaves
     fn build_nest(&self) {
         println!("swallows build nests under the eaves")
     }
}

// trait
trait MigrantBird {
     //Leave it to the respective types to implement
     fn migrant(&self) -> String;
}

// Implement the migrant method of Trait feature for WildGoose
impl MigrantBird for WildGoose {
     fn migrant(&self) -> String {
         "Geese fly in a V-shaped formation".to_string()
     }
}

// Implement the migrant method of Trait for Swallow
impl MigrantBird for Swallow {
     fn migrant(&self) -> String {
         "swallow fly fast, but have to rest frequently".to_string()
     }
}


// 3. Quest
struct WildGoose {
    color: String,
}

struct Swallow {
    color: String,
}

trait Tweet {
    // Please complete the definition of Trait method
    fn tweet(&self) -> String;
}

impl Tweet for WildGoose {
    fn tweet(&self) -> String {
        "ga ga".to_string()
    }
}

impl Tweet for Swallow {
    fn tweet(&self) -> String {
        "ji ji zha zha".to_string()
    }
}
