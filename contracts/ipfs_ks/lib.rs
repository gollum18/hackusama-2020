#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;


#[ink::contract(version = "0.1.0")]
mod ipfs_ks {

    use ipfs_coin::IpfsCoin;

    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_core::storage2::{
        collections::HashMap as StorageHashMap,
        collections::Vec as StorageVector,
        lazy::Lazy
    };

    #[ink(event)]
    struct RequestUploadEvent {
        account: AccountId,
        hash: Hash,
    }

    #[ink(event)]
    struct RequestRemoveEvent {
        account: AccountId,
        hash: Hash,
    }

    #[ink(event)]
    struct RequestModifyEvent {
        account: AccountId,
        hash: Hash
    }

    #[ink(event)]
    struct RequestReadEvent {
        account: AccountId,
        hash: Hash,
    }

    enum IpfsPermission {
        READ,
        WRITE
    }

    struct IpfsFile {
        author: AccountId,
        perms: StorageHashMap<AccountId, (bool, bool)>,
        origin_ts: Timestamp,
    }

    impl IpfsFile {
        fn new(author: &AccountId) -> Self {
            let now = Self::env.block_timestamp();
            Self {
                author: *author,
                perms: StorageHashMap::new(),
                origin_ts: now,
            }
        }

        fn is_author(&self, account: &AccountId) -> bool {
            *account == *self.author
        }

        fn has_permission(
            &self, 
            account: &AccountId, 
            perm: IpfsPermission) 
        -> bool {
            let mut result = false;
            if *self.perms.contains_key(*account) {
                result = match perm {
                    READ => *self.perms.get(*account).0,
                    WRITE => *self.perms.get(*account).1,
                    _ => false
                }
            }
            result
        }

        fn add_permission(
            &self,
            account: &AccountId,
            perm: IpfsPermission) 
        -> bool {
            let mut result = false;
            if *self.perms.contains_key(*account) {
                let acct_perms = *self.perms.take(*account);
                let new_acct_perms = match perm {
                    READ => (true, acct_perms[1]]),
                    WRITE => (acct_perms[0], true),
                    _ => return false
                }
                *self.perms.insert(*account, new_acct_perms);
                result = true;
            } else {
                let new_acct_perms = match perm {
                    READ => (true, false),
                    WRITE => (false, true),
                    _ => return false
                }
                *self.perms.insert(*account, new_acct_perms);
                result = true;
            }
            result
        }

        fn remove_permission(
            &self,
            account: &AccountId,
            perm: IpfsPermission) 
        -> bool {
            let mut result = false;
            if *self.perms.contains_key(*account) {
                let acct_perms = *self.perms.take(*account);
                let new_acct_perms = match perm {
                    READ => (false, acct_perms[1]]),
                    WRITE => (acct_perms[0], false),
                    _ => return false
                }
                *self.perms.insert(*account, new_acct_perms);
                result = true;
            }
            result
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    struct IpfsKs {
        ipfs_coin: Lazy<IpfsCoin>,
        files: StorageHashMap<Hash, IpfsFile>,
    }

    impl IpfsKs {
        #[ink(constructor)]
        fn new(
            ipfs_coin_code_hash: Hash, 
            initial_supply: Balance) 
        -> Self {
            let total_balance = Self::env().balance();
            let ipfs_coin = IpfsCoin::new(initial_supply, &Self::env().caller())
                .endowment(total_balance/2)
                .using_code(ipfs_coin_code_hash)
                .instantiate()
                .expect("failed at instantiating the `IpfsCoin` contract");
            Self {
                ipfs_coin: Lazy::new(ipfs_coin),
                files: StorageHashMap::new()
            }
        }

        #[ink(message)]
        fn request_upload(&mut self, hash: &Hash) -> bool {
            let caller = Self::env().caller();
            Self::env.emit_event(RequestUploadEvent {
                caller,
                hash,
            });
            true
        }
        
        #[ink(message)]
        fn request_remove(&mut self, hash: &Hash) {
            let caller = Self::env().caller();
            Self::env.emit_event(RequestRemoveEvent {
                caller,
                hash,
            });
            true
        }
        
        #[ink(message)]
        fn request_modify(&mut self, hash: &Hash) {
            let caller = Self::env().caller();
            Self::env.emit_event(RequestModifyEvent {
                caller,
                hash,
            });
            true
        }
        
        #[ink(message)]
        fn request_read(&self, hash: &Hash) {
            let caller = Self::env().caller();
            Self::env.emit_event(RequestReadEvent {
                caller,
                hash,
            });
            true
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

    }
}
