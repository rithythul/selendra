use crate::{Runtime, RuntimeEvent, Balances};
use frame_support::parameter_types;

parameter_types! {
    pub const MaxTransactionsPerBatch: u32 = 1000;
    pub const ProofVerificationGas: u32 = 500_000;
    pub const MinStake: Balance = 1000 * TOKENS;
}

impl pallet_zk_rollup::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxTransactionsPerBatch = MaxTransactionsPerBatch;
    type ProofVerificationGas = ProofVerificationGas;
    type MinStake = MinStake;
}
