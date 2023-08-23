
//! Autogenerated weights for `runtime_parachains::paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `s-02-v-02`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! EXECUTION: `Some(Wasm)`, WASM-EXECUTION: `Compiled`, CHAIN: `Some("selendra-dev")`, DB CACHE: 1024

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
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras_inherent::WeightInfo for WeightInfo<T> {
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `Parascheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasessionInfo::Sessions` (r:1 w:0)
	/// Proof: `ParasessionInfo::Sessions` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:1)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::BackersOnDisputes` (r:1 w:1)
	/// Proof: `ParasDisputes::BackersOnDisputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:1 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasessionInfo::AccountKeys` (r:1 w:0)
	/// Proof: `ParasessionInfo::AccountKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ActiveEra` (r:1 w:0)
	/// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasRewardPoints` (r:1 w:1)
	/// Proof: `Staking::ErasRewardPoints` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailability` (r:2 w:1)
	/// Proof: `ParaInclusion::PendingAvailability` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Parachains` (r:1 w:0)
	/// Proof: `Paras::Parachains` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailabilityCommitments` (r:1 w:1)
	/// Proof: `ParaInclusion::PendingAvailabilityCommitments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `Parascheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ParathreadQueue` (r:1 w:1)
	/// Proof: `Parascheduler::ParathreadQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::Scheduled` (r:1 w:1)
	/// Proof: `Parascheduler::Scheduled` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `Parascheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[10, 200]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `51021`
		//  Estimated: `56961 + v * (23 ±0)`
		// Minimum execution time: 669_878_000 picoseconds.
		Weight::from_parts(249_448_872, 0)
			.saturating_add(Weight::from_parts(0, 56961))
			// Standard Error: 28_070
			.saturating_add(Weight::from_parts(42_614_965, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(28))
			.saturating_add(T::DbWeight::get().writes(15))
			.saturating_add(Weight::from_parts(0, 23).saturating_mul(v.into()))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `Parascheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailability` (r:2 w:1)
	/// Proof: `ParaInclusion::PendingAvailability` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Parachains` (r:1 w:0)
	/// Proof: `Paras::Parachains` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailabilityCommitments` (r:1 w:1)
	/// Proof: `ParaInclusion::PendingAvailabilityCommitments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasessionInfo::AccountKeys` (r:1 w:0)
	/// Proof: `ParasessionInfo::AccountKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ActiveEra` (r:1 w:0)
	/// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasRewardPoints` (r:1 w:1)
	/// Proof: `Staking::ErasRewardPoints` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:0)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `Parascheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ParathreadQueue` (r:1 w:1)
	/// Proof: `Parascheduler::ParathreadQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::Scheduled` (r:1 w:1)
	/// Proof: `Parascheduler::Scheduled` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `Parascheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::AvailabilityBitfields` (r:0 w:1)
	/// Proof: `ParaInclusion::AvailabilityBitfields` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Included` (r:0 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn enter_bitfields() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42854`
		//  Estimated: `48794`
		// Minimum execution time: 260_119_000 picoseconds.
		Weight::from_parts(268_374_000, 0)
			.saturating_add(Weight::from_parts(0, 48794))
			.saturating_add(T::DbWeight::get().reads(26))
			.saturating_add(T::DbWeight::get().writes(16))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `Parascheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailability` (r:2 w:1)
	/// Proof: `ParaInclusion::PendingAvailability` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Parachains` (r:1 w:0)
	/// Proof: `Paras::Parachains` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailabilityCommitments` (r:1 w:1)
	/// Proof: `ParaInclusion::PendingAvailabilityCommitments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasessionInfo::AccountKeys` (r:1 w:0)
	/// Proof: `ParasessionInfo::AccountKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ActiveEra` (r:1 w:0)
	/// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasRewardPoints` (r:1 w:1)
	/// Proof: `Staking::ErasRewardPoints` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:0)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `Parascheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ParathreadQueue` (r:1 w:1)
	/// Proof: `Parascheduler::ParathreadQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::Scheduled` (r:1 w:1)
	/// Proof: `Parascheduler::Scheduled` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `Parascheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `ParasDisputes::Included` (r:0 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `v` is `[101, 200]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42889`
		//  Estimated: `48829`
		// Minimum execution time: 5_020_680_000 picoseconds.
		Weight::from_parts(1_318_806_394, 0)
			.saturating_add(Weight::from_parts(0, 48829))
			// Standard Error: 128_190
			.saturating_add(Weight::from_parts(38_534_639, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(29))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	/// Storage: `ParaInherent::Included` (r:1 w:1)
	/// Proof: `ParaInherent::Included` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::ParentHash` (r:1 w:0)
	/// Proof: `System::ParentHash` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `ParasShared::CurrentSessionIndex` (r:1 w:0)
	/// Proof: `ParasShared::CurrentSessionIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::AvailabilityCores` (r:1 w:1)
	/// Proof: `Parascheduler::AvailabilityCores` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Babe::AuthorVrfRandomness` (r:1 w:0)
	/// Proof: `Babe::AuthorVrfRandomness` (`max_values`: Some(1), `max_size`: Some(33), added: 528, mode: `MaxEncodedLen`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInherent::OnChainVotes` (r:1 w:1)
	/// Proof: `ParaInherent::OnChainVotes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Frozen` (r:1 w:0)
	/// Proof: `ParasDisputes::Frozen` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailability` (r:2 w:1)
	/// Proof: `ParaInclusion::PendingAvailability` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Parachains` (r:1 w:0)
	/// Proof: `Paras::Parachains` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParaInclusion::PendingAvailabilityCommitments` (r:1 w:1)
	/// Proof: `ParaInclusion::PendingAvailabilityCommitments` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasessionInfo::AccountKeys` (r:1 w:0)
	/// Proof: `ParasessionInfo::AccountKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::Validators` (r:1 w:0)
	/// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Staking::ActiveEra` (r:1 w:0)
	/// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
	/// Storage: `Staking::ErasRewardPoints` (r:1 w:1)
	/// Proof: `Staking::ErasRewardPoints` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:1 w:1)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasDisputes::Disputes` (r:1 w:0)
	/// Proof: `ParasDisputes::Disputes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::SessionStartBlock` (r:1 w:0)
	/// Proof: `Parascheduler::SessionStartBlock` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ParathreadQueue` (r:1 w:1)
	/// Proof: `Parascheduler::ParathreadQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::Scheduled` (r:1 w:1)
	/// Proof: `Parascheduler::Scheduled` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Parascheduler::ValidatorGroups` (r:1 w:0)
	/// Proof: `Parascheduler::ValidatorGroups` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeHash` (r:1 w:0)
	/// Proof: `Paras::FutureCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeRestrictionSignal` (r:1 w:0)
	/// Proof: `Paras::UpgradeRestrictionSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::ParaLifecycles` (r:1 w:0)
	/// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:0)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `ParasDisputes::Included` (r:0 w:1)
	/// Proof: `ParasDisputes::Included` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42916`
		//  Estimated: `48856`
		// Minimum execution time: 26_253_599_000 picoseconds.
		Weight::from_parts(27_679_657_000, 0)
			.saturating_add(Weight::from_parts(0, 48856))
			.saturating_add(T::DbWeight::get().reads(31))
			.saturating_add(T::DbWeight::get().writes(15))
	}
}
