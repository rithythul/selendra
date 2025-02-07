use sp_core::H256;
use sp_runtime::traits::Hash;
use sp_std::{collections::btree_map::BTreeMap, prelude::*};

#[derive(Clone, Debug)]
pub struct Account {
    pub balance: u128,
    pub nonce: u64,
}

pub struct StateTree {
    accounts: BTreeMap<Vec<u8>, Account>,
    root: H256,
}

impl StateTree {
    pub fn new() -> Self {
        Self {
            accounts: BTreeMap::new(),
            root: H256::zero(),
        }
    }

    pub fn get_account(&self, address: &[u8]) -> Option<&Account> {
        self.accounts.get(address)
    }

    pub fn update_account(&mut self, address: Vec<u8>, account: Account) {
        self.accounts.insert(address, account);
        self.update_root();
    }

    pub fn get_root(&self) -> H256 {
        self.root
    }

    fn update_root(&mut self) {
        // TODO: Implement Merkle tree root calculation
        // This is a placeholder that sets a dummy root
        self.root = H256::zero();
    }

    pub fn verify_inclusion(&self, address: &[u8], account: &Account) -> bool {
        // TODO: Implement Merkle proof verification
        // This is a placeholder that always returns true if account exists
        self.accounts.get(address).map_or(false, |a| a == account)
    }
}

#[derive(Clone, Debug)]
pub struct StateUpdate {
    pub address: Vec<u8>,
    pub old_account: Option<Account>,
    pub new_account: Account,
}

pub struct StateTransition {
    pub updates: Vec<StateUpdate>,
    pub old_root: H256,
    pub new_root: H256,
}

impl StateTransition {
    pub fn new(updates: Vec<StateUpdate>, old_root: H256, new_root: H256) -> Self {
        Self {
            updates,
            old_root,
            new_root,
        }
    }

    pub fn verify(&self, state: &StateTree) -> bool {
        // Verify old root matches
        if state.get_root() != self.old_root {
            return false;
        }

        // Verify all updates are valid
        for update in &self.updates {
            let current_account = state.get_account(&update.address);
            if current_account != update.old_account.as_ref() {
                return false;
            }
        }

        true
    }
}
