#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod ipfs_ks {

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_core::storage2::{
        collections::{
            HashMap as InkHashMap
        }
    };

    pub enum TxOp {
        Create,
        Read,
        Write
    }

    #[ink(event)]
    struct FileCreated {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        hash: Hash,
        #[ink(topic)]
        ts: Timestamp
    }

    #[ink(event)]
    struct FileRemoved {
        #[ink(topic)]
        hash: Hash,
        #[ink(topic)]
        ts: Timestamp
    }

    #[ink(event)]
    struct FileModified {
        #[ink(topic)]
        hash: Hash,
        #[ink(topic)]
        ts: Timestamp
    }

    #[ink(event)]
    struct FileRead {
        #[ink(topic)]
        hash: Hash,
        #[ink(topic)]
        ts: Timestamp
    }

    #[ink(storage)]
    struct IpfsKs {
        balances: InkHashMap<AccountId, Balance>,
        // first u32 is location, 2nd is number of ops performed on it
        files: InkHashMap<Hash, (AccountId, i32)>
    }

    impl IpfsKs {

        #[ink(constructor)]
        fn new() -> Self {
            Self {
                balances: InkHashMap::new(),
                files: InkHashMap::new()
            }
        }

        #[ink(message)]
        fn num_users(&self) -> u32 {
            self.balances.len()
        }

        #[ink[message]]
        fn num_files(&self) -> u32 {
            self.files.len()
        }

        #[ink(message)]
        fn is_user_registered(&self, account: AccountId) -> bool {
            self.balances.get(&account).is_some()
        }

        #[ink(message)]
        fn register(&mut self, account: AccountId, initial_balance: Balance) -> bool {
            if self.is_user_registered(*&account) {
                return false
            }
            self.balances.insert(account, initial_balance);
            true
        }
        
        #[ink(message)]
        fn deposit(&mut self, value: Balance) -> bool {
            let caller = self.env().caller();

            if !self.balances.get(&caller).is_some() {
                return false
            }

            let balance = self.balance_or_zero(&caller);
            self.balances.insert(caller, balance + value);

            true
        }
        
        #[ink(message)]
        fn withdraw(&mut self, value: Balance) -> Balance {
            let caller = self.env().caller();

            if !self.balances.get(&caller).is_some() {
                return 0
            }

            let balance = self.balance_or_zero(&caller);
            if value > balance {
                return 0
            }
            self.balances.insert(caller, balance - value);

            value
        }
        
        #[ink(message)]
        fn balance(&self) -> Balance {
            let caller = self.env().caller();

            if !self.balances.get(&caller).is_some() {
                return 0
            }

            self.balance_or_zero(&caller)
        }

        fn charge(&mut self, account: &AccountId, hash: &Hash, op: TxOp) -> bool {
            let base_cost: f32 = match op {
                TxOp::Create => 3., 
                TxOp::Read => 2.,
                TxOp::Write => 3.
            };
            let t = self.files.get(&hash).unwrap().1 as f32;
            let cost = (base_cost + 1.07 * t) as u128;
            let balance = self.balance_or_zero(account);
            if cost > balance {
                return false
            }
            self.balances.insert(*account, balance - cost);
            // the difference here should be added to the contract accounts balance
            true
        }

        #[ink(message)]
        fn add_file(&mut self, hash: Hash) -> bool {
            let caller = self.env().caller();

            if !self.balances.get(&caller).is_some() {
                return false
            }

            if self.files.get(&hash).is_some() {
                return false
            }

            if !self.charge(&caller, &hash, TxOp::Create) {
                return false
            }

            let timestamp = self.env().block_timestamp();
            self.files.insert(hash, (caller, 0));
            self.env().emit_event(FileCreated {
                owner: caller,
                hash: hash,
                ts: timestamp
            });

            true
        }

        #[ink(message)]
        fn remove_file(&mut self, hash: Hash) -> bool  {
            let caller = self.env().caller();

            if !self.balances.get(&caller).is_some() {
                return false
            }

            if !self.files.get(&hash).is_some() {
                return false
            }

            let timestamp = self.env().block_timestamp();
            self.env().emit_event(FileRemoved {
                hash: hash,
                ts: timestamp
            });

            true
        }

        #[ink(message)]
        fn write_file(&mut self, hash: Hash) -> bool {
            let caller = self.env().caller();

            if !self.balances.get(&caller).is_some() {
                return false
            }

            if !self.files.get(&hash).is_some() {
                return false
            }

            if !self.charge(&caller, &hash, TxOp::Write) {
                return false
            }

            let stats = *self.files.get(&hash).unwrap();
            self.files.insert(hash, (stats.0, stats.1 + 1));

            let timestamp = self.env().block_timestamp();
            self.env().emit_event(FileModified {
                hash: hash,
                ts: timestamp
            });

            true
        }

        #[ink(message)]
        fn read_file(&mut self, hash: Hash) -> bool {
            let caller = self.env().caller();

            if !self.balances.get(&caller).is_some() {
                return false
            }

            if !self.files.get(&hash).is_some() {
                return false
            }

            if !self.charge(&caller, &hash, TxOp::Read) {
                return false
            }

            let stats = *self.files.get(&hash).unwrap();
            self.files.insert(hash, (stats.0, stats.1 + 1));

            let timestamp = self.env().block_timestamp();
            self.env().emit_event(FileRead {
                hash: hash,
                ts: timestamp
            });

            true
        }

        fn balance_or_zero(&self, owner: &AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_core::env;

        /*fn run_test<F>(test_fn: F) 
        where
            F: FnOnce()
        {
            env::test::run_test::<env::DefaultEnvTypes, _>(|_| {
                test_fn();
                Ok(())
            })  
            .unwrap()  
        }

        #[test]
        fn test_constructor() {
            run_test(|| {
                let ipfs_ks = IpfsKs::new();
                assert_eq!(ipfs_ks.num_users(), 0);
                assert_eq!(ipfs_ks.num_files(), 0);
            })
        }

        #[test]
        fn test_registration {
            run_test(|| {

            })
        }

        #[test]
        fn test_good_deposit() {
            run_test(|| {

            })
        }

        #[test]
        fn test_bad_deposit() {
            run_test(|| {

            })
        }

        #[test]
        fn test_good_withdrawal() {
            run_test(|| {

            })
        }

        #[test]
        fn test_bad_withdrawal() {
            run_test(|| {

            })
        }

        #[test]
        test_balance() {
            run_test(|| {

            })
        }

        #[test]
        fn test_create_file() {
            run_test(|| {

            })
        }

        #[test]
        fn test_delete_file() {
            run_test(|| {

            })
        }

        #[test]
        fn test_read_file() {
            run_test(|| {

            })
        }

        #[test]
        fn test_write_file() {
            run_test(|| {

            })
        }

        #[test]
        fn test_charge_function() {
            run_test(|| {

            })
        }*/
    
    }
}
