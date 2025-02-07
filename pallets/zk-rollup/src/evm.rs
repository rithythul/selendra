use frame_support::traits::Currency;
use sp_core::{H160, H256, U256};
use sp_std::prelude::*;

use crate::state::{Account, StateTree};

// EVM Account structure
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
pub struct EvmAccount {
    pub nonce: U256,
    pub balance: U256,
    pub code_hash: H256,
    pub code: Vec<u8>,
    pub storage: BTreeMap<H256, H256>,
}

// EVM Transaction structure
#[derive(Clone, Debug, Encode, Decode)]
pub struct EvmTransaction {
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub action: TransactionAction,
    pub value: U256,
    pub input: Vec<u8>,
    pub signature: TransactionSignature,
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum TransactionAction {
    Call(H160),
    Create,
}

#[derive(Clone, Debug, Encode, Decode)]
pub struct TransactionSignature {
    pub v: u64,
    pub r: H256,
    pub s: H256,
}

// EVM State Manager
pub struct EvmStateManager {
    state: StateTree,
    block_number: u64,
    block_hash: H256,
    block_timestamp: u64,
    chain_id: u64,
}

impl EvmStateManager {
    pub fn new(state: StateTree, chain_id: u64) -> Self {
        Self {
            state,
            block_number: 0,
            block_hash: H256::zero(),
            block_timestamp: 0,
            chain_id,
        }
    }

    pub fn get_account(&self, address: &H160) -> Option<EvmAccount> {
        self.state.get_account(&address.to_fixed_bytes())
            .map(|account| self.convert_to_evm_account(account))
    }

    pub fn set_account(&mut self, address: H160, account: EvmAccount) {
        let state_account = self.convert_from_evm_account(account);
        self.state.update_account(address.to_fixed_bytes().to_vec(), state_account);
    }

    pub fn execute_transaction(&mut self, tx: EvmTransaction) -> Result<EvmExecutionResult, EvmError> {
        // Verify transaction
        self.verify_transaction(&tx)?;

        // Create execution context
        let context = self.create_execution_context(&tx);

        // Execute the EVM
        let result = self.execute_evm(context, tx)?;

        // Apply state changes
        self.apply_state_changes(result.state_changes);

        Ok(result)
    }

    fn verify_transaction(&self, tx: &EvmTransaction) -> Result<(), EvmError> {
        // Verify nonce
        let sender = self.recover_sender(tx)?;
        let account = self.get_account(&sender)
            .ok_or(EvmError::AccountNotFound)?;
        
        if tx.nonce != account.nonce {
            return Err(EvmError::InvalidNonce);
        }

        // Verify balance
        let required_balance = tx.gas_price
            .checked_mul(tx.gas_limit.into())
            .and_then(|gas_cost| gas_cost.checked_add(tx.value))
            .ok_or(EvmError::BalanceOverflow)?;

        if account.balance < required_balance {
            return Err(EvmError::InsufficientBalance);
        }

        Ok(())
    }

    fn execute_evm(&mut self, context: ExecutionContext, tx: EvmTransaction) -> Result<EvmExecutionResult, EvmError> {
        let config = Config::istanbul();
        let precompiles = Precompiles::istanbul();
        
        let mut executor = StackExecutor::new(
            &context.state,
            config,
            &precompiles,
            &context.sender,
            &context.receiver,
            0, // Depth
            true, // Static
            false, // Read-only
        );

        match tx.action {
            TransactionAction::Call(address) => {
                executor.transact_call(
                    context.sender,
                    address,
                    tx.value,
                    tx.input,
                    tx.gas_limit.as_u64(),
                )
            }
            TransactionAction::Create => {
                executor.transact_create(
                    context.sender,
                    tx.value,
                    tx.input,
                    tx.gas_limit.as_u64(),
                )
            }
        }
    }

    fn apply_state_changes(&mut self, changes: Vec<StateChange>) {
        for change in changes {
            match change {
                StateChange::AccountUpdate { address, account } => {
                    self.set_account(address, account);
                }
                StateChange::StorageUpdate { address, key, value } => {
                    if let Some(mut account) = self.get_account(&address) {
                        account.storage.insert(key, value);
                        self.set_account(address, account);
                    }
                }
                StateChange::CodeUpdate { address, code } => {
                    if let Some(mut account) = self.get_account(&address) {
                        account.code = code;
                        account.code_hash = H256::from_slice(sp_io::hashing::keccak_256(&code));
                        self.set_account(address, account);
                    }
                }
            }
        }
    }

    fn convert_to_evm_account(&self, account: Account) -> EvmAccount {
        EvmAccount {
            nonce: U256::from(account.nonce),
            balance: U256::from(account.balance),
            code_hash: H256::zero(), // Default for non-contract accounts
            code: Vec::new(),
            storage: BTreeMap::new(),
        }
    }

    fn convert_from_evm_account(&self, account: EvmAccount) -> Account {
        Account {
            nonce: account.nonce.as_u64(),
            balance: account.balance.as_u128(),
        }
    }
}

#[derive(Debug)]
pub enum EvmError {
    AccountNotFound,
    InvalidNonce,
    InsufficientBalance,
    BalanceOverflow,
    ExecutionError(String),
}

pub struct EvmExecutionResult {
    pub success: bool,
    pub gas_used: u64,
    pub return_data: Vec<u8>,
    pub state_changes: Vec<StateChange>,
    pub logs: Vec<Log>,
}

#[derive(Clone, Debug)]
pub enum StateChange {
    AccountUpdate {
        address: H160,
        account: EvmAccount,
    },
    StorageUpdate {
        address: H160,
        key: H256,
        value: H256,
    },
    CodeUpdate {
        address: H160,
        code: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub struct Log {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}
