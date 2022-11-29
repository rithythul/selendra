
//! Autogenerated weights for `runtime_parachains::paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `selendra-benchamarking`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/runtime_parachains_paras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras_inherent::WeightInfo for WeightInfo<T> {
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParasessionInfo Sessions (r:1 w:0)
	// Storage: ParasDisputes Disputes (r:1 w:1)
	// Storage: ParasDisputes Included (r:1 w:1)
	// Storage: ParasDisputes SpamSlots (r:1 w:1)
	// Storage: Parascheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParasessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: Parascheduler SessionStartBlock (r:1 w:0)
	// Storage: Parascheduler ParathreadQueue (r:1 w:1)
	// Storage: Parascheduler Scheduled (r:1 w:1)
	// Storage: Parascheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[10, 200]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		(842_104_000 as Weight)
			// Standard Error: 463_000
			.saturating_add((95_831_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(29 as Weight))
			.saturating_add(T::DbWeight::get().writes(18 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: Parascheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParasessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:1 w:0)
	// Storage: Parascheduler SessionStartBlock (r:1 w:0)
	// Storage: Parascheduler ParathreadQueue (r:1 w:1)
	// Storage: Parascheduler Scheduled (r:1 w:1)
	// Storage: Parascheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInclusion AvailabilityBitfields (r:0 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_bitfields() -> Weight {
		(842_132_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(26 as Weight))
			.saturating_add(T::DbWeight::get().writes(17 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: Parascheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParasessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:2 w:0)
	// Storage: Parascheduler SessionStartBlock (r:1 w:0)
	// Storage: Parascheduler ParathreadQueue (r:1 w:1)
	// Storage: Parascheduler Scheduled (r:1 w:1)
	// Storage: Parascheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[101, 200]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_324_000
			.saturating_add((123_201_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(29 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: Parascheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParasessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:2 w:0)
	// Storage: Parascheduler SessionStartBlock (r:1 w:0)
	// Storage: Parascheduler ParathreadQueue (r:1 w:1)
	// Storage: Parascheduler Scheduled (r:1 w:1)
	// Storage: Parascheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Paras FutureCodeHash (r:1 w:0)
	// Storage: Paras UpgradeRestrictionSignal (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		(89_499_206_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(31 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
}