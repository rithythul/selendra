# Selendra Runtime Specification

## Table of Contents
1. [Pallet Documentation](#pallet-documentation)
2. [State Transitions](#state-transitions)
3. [Storage Layout](#storage-layout)
4. [Custom Types](#custom-types)
5. [Runtime Upgrades](#runtime-upgrades)

## Pallet Documentation

### System Pallet
```rust
pub struct System<T: Config> {
    // Account information
    pub struct AccountInfo {
        nonce: Index,
        consumers: RefCount,
        providers: RefCount,
        data: AccountData<Balance>,
    }
    
    // Block information
    pub struct BlockInfo {
        height: BlockNumber,
        timestamp: Moment,
        author: AccountId,
    }
}

impl<T: Config> Pallet<T> {
    // Core system functions
    pub fn account(who: &T::AccountId) -> AccountInfo<T>;
    pub fn block_number() -> T::BlockNumber;
    pub fn block_hash(n: T::BlockNumber) -> T::Hash;
}
```

### Balances Pallet
```rust
pub struct Balances<T: Config> {
    // Account balance information
    pub struct AccountData<Balance> {
        free: Balance,
        reserved: Balance,
        frozen: Balance,
    }
    
    // Transfer event
    pub enum Event<T: Config> {
        Transfer(T::AccountId, T::AccountId, Balance),
        Deposit(T::AccountId, Balance),
        Withdraw(T::AccountId, Balance),
    }
}

impl<T: Config> Pallet<T> {
    // Balance operations
    pub fn transfer(
        source: &T::AccountId,
        dest: &T::AccountId,
        value: T::Balance
    ) -> DispatchResult;
    
    pub fn get_balance(who: &T::AccountId) -> T::Balance;
}
```

### Staking Pallet
```rust
pub struct Staking<T: Config> {
    // Validator information
    pub struct ValidatorInfo {
        stash: AccountId,
        commission: Perbill,
        blocked: bool,
    }
    
    // Nomination information
    pub struct Nomination {
        targets: Vec<AccountId>,
        submitted_in: EraIndex,
        suppressed: bool,
    }
}

impl<T: Config> Pallet<T> {
    // Staking operations
    pub fn bond(controller: T::AccountId, value: T::Balance) -> DispatchResult;
    pub fn nominate(targets: Vec<T::AccountId>) -> DispatchResult;
    pub fn chill() -> DispatchResult;
}
```

## State Transitions

### Account State Transitions
```rust
pub enum AccountState {
    // Account creation
    New {
        balance: Balance,
        nonce: Index,
    },
    
    // Account modification
    Modified {
        balance_change: Balance,
        nonce_change: Index,
    },
    
    // Account deletion
    Deleted,
}

impl AccountState {
    pub fn apply(
        &self,
        account: &mut AccountInfo
    ) -> Result<(), Error> {
        match self {
            AccountState::New { balance, nonce } => {
                account.balance = *balance;
                account.nonce = *nonce;
            },
            AccountState::Modified { balance_change, nonce_change } => {
                account.balance = account.balance.saturating_add(*balance_change);
                account.nonce = account.nonce.saturating_add(*nonce_change);
            },
            AccountState::Deleted => {
                account.balance = Zero::zero();
                account.nonce = Zero::zero();
            },
        }
        Ok(())
    }
}
```

### Block State Transitions
```rust
pub struct BlockTransition {
    pub header: Header,
    pub extrinsics: Vec<UncheckedExtrinsic>,
    pub state_changes: Vec<StateChange>,
}

impl BlockTransition {
    pub fn apply(
        &self,
        state: &mut RuntimeState
    ) -> Result<(), Error> {
        // Verify header
        self.verify_header(&state)?;
        
        // Apply extrinsics
        for ext in &self.extrinsics {
            ext.apply(state)?;
        }
        
        // Apply state changes
        for change in &self.state_changes {
            change.apply(state)?;
        }
        
        Ok(())
    }
}
```

## Storage Layout

### Storage Structure
```rust
pub struct StorageLayout {
    // Metadata about storage items
    pub struct StorageMetadata {
        prefix: Vec<u8>,
        modifier: StorageModifier,
        ty: StorageEntryType,
        default: Vec<u8>,
        documentation: Vec<&'static str>,
    }
    
    // Storage entry types
    pub enum StorageEntryType {
        Plain(Scale),
        Map {
            hasher: StorageHasher,
            key: Scale,
            value: Scale,
        },
        DoubleMap {
            hasher1: StorageHasher,
            key1: Scale,
            hasher2: StorageHasher,
            key2: Scale,
            value: Scale,
        },
    }
}
```

### Storage Operations
```rust
pub trait Storage {
    // Basic operations
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn put(&mut self, key: &[u8], value: &[u8]);
    fn kill(&mut self, key: &[u8]);
    
    // Batch operations
    fn batch_verify(
        &self,
        keys: &[Vec<u8>],
        values: &[Option<Vec<u8>>],
    ) -> bool;
    
    // Merkle proof
    fn prove_read(
        &self,
        key: &[u8],
    ) -> Option<Vec<Vec<u8>>>;
}
```

## Custom Types

### Basic Types
```rust
// Account types
pub type AccountId = [u8; 32];
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Hash = [u8; 32];
pub type Moment = u64;

// Custom structs
pub struct Asset {
    id: AssetId,
    owner: AccountId,
    details: AssetDetails,
}

pub struct AssetDetails {
    supply: Balance,
    min_balance: Balance,
    max_zombies: u32,
}
```

### Transaction Types
```rust
pub enum TransactionPriority {
    Normal,
    Operational,
    Mandatory,
}

pub struct TransactionValidity {
    priority: TransactionPriority,
    requires: Vec<Vec<u8>>,
    provides: Vec<Vec<u8>>,
    longevity: u64,
    propagate: bool,
}
```

### Event Types
```rust
pub enum Event {
    System(SystemEvent),
    Balances(BalancesEvent),
    Staking(StakingEvent),
    Custom(CustomEvent),
}

pub struct EventRecord<T: Config> {
    phase: Phase,
    event: Event<T>,
    topics: Vec<T::Hash>,
}
```

## Runtime Upgrades

### Upgrade Process
```rust
pub struct RuntimeUpgrade {
    pub struct UpgradeSchedule {
        version: RuntimeVersion,
        spec_version: u32,
        spec_name: &'static str,
    }
    
    pub enum UpgradeType {
        Scheduled(BlockNumber),
        Immediate,
        Emergency,
    }
}

impl RuntimeUpgrade {
    pub fn schedule_upgrade(
        schedule: UpgradeSchedule,
        upgrade_type: UpgradeType,
    ) -> Result<(), Error> {
        // Validation
        Self::validate_upgrade(&schedule)?;
        
        // Schedule
        match upgrade_type {
            UpgradeType::Scheduled(block) => {
                Self::schedule_at(block, schedule)
            },
            UpgradeType::Immediate => {
                Self::apply_immediate(schedule)
            },
            UpgradeType::Emergency => {
                Self::apply_emergency(schedule)
            },
        }
    }
    
    pub fn validate_upgrade(
        schedule: &UpgradeSchedule,
    ) -> Result<(), Error> {
        // Version checks
        ensure!(
            schedule.spec_version > CURRENT_VERSION,
            Error::InvalidVersion
        );
        
        // Compatibility checks
        Self::check_compatibility(schedule)?;
        
        Ok(())
    }
}
```

### Migration Handling
```rust
pub trait StorageMigration {
    fn migrate_storage() -> Weight;
}

pub struct Migration<T: Config> {
    pub fn on_runtime_upgrade() -> Weight {
        // Pre-upgrade hooks
        Self::pre_upgrade()?;
        
        // Apply migrations
        let weight = Self::migrate_storage();
        
        // Post-upgrade hooks
        Self::post_upgrade()?;
        
        weight
    }
    
    pub fn pre_upgrade() -> Result<(), Error> {
        // Backup critical state
        Self::backup_state()?;
        
        // Verify storage integrity
        Self::verify_storage()?;
        
        Ok(())
    }
    
    pub fn post_upgrade() -> Result<(), Error> {
        // Verify migration success
        Self::verify_migration()?;
        
        // Update version
        Self::update_version()?;
        
        Ok(())
    }
}
```
