
//! Autogenerated weights for `pallet_xcm`
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
// --pallet=pallet_xcm
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/pallet_xcm.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xcm`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm::WeightInfo for WeightInfo<T> {
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `XcmPallet::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SafeXcmVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn send() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `527`
		//  Estimated: `3992`
		// Minimum execution time: 27_792_000 picoseconds.
		Weight::from_parts(28_284_000, 0)
			.saturating_add(Weight::from_parts(0, 3992))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	fn teleport_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_282_000 picoseconds.
		Weight::from_parts(16_541_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn reserve_transfer_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 16_090_000 picoseconds.
		Weight::from_parts(16_471_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn execute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `XcmPallet::SupportedVersion` (r:0 w:1)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_213_000 picoseconds.
		Weight::from_parts(7_404_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmPallet::SafeXcmVersion` (r:0 w:1)
	/// Proof: `XcmPallet::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn force_default_xcm_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_205_000 picoseconds.
		Weight::from_parts(2_334_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmPallet::VersionNotifiers` (r:1 w:1)
	/// Proof: `XcmPallet::VersionNotifiers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::QueryCounter` (r:1 w:1)
	/// Proof: `XcmPallet::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `XcmPallet::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SafeXcmVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::Queries` (r:0 w:1)
	/// Proof: `XcmPallet::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_subscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `527`
		//  Estimated: `3992`
		// Minimum execution time: 30_898_000 picoseconds.
		Weight::from_parts(31_609_000, 0)
			.saturating_add(Weight::from_parts(0, 3992))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `XcmPallet::VersionNotifiers` (r:1 w:1)
	/// Proof: `XcmPallet::VersionNotifiers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `XcmPallet::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SafeXcmVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::Queries` (r:0 w:1)
	/// Proof: `XcmPallet::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn force_unsubscribe_version_notify() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `4264`
		// Minimum execution time: 33_623_000 picoseconds.
		Weight::from_parts(34_014_000, 0)
			.saturating_add(Weight::from_parts(0, 4264))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `XcmPallet::XcmExecutionSuspended` (r:0 w:1)
	/// Proof: `XcmPallet::XcmExecutionSuspended` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn force_suspension() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_214_000 picoseconds.
		Weight::from_parts(2_324_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `XcmPallet::SupportedVersion` (r:4 w:2)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_supported_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `229`
		//  Estimated: `11119`
		// Minimum execution time: 12_954_000 picoseconds.
		Weight::from_parts(13_075_000, 0)
			.saturating_add(Weight::from_parts(0, 11119))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `XcmPallet::VersionNotifiers` (r:4 w:2)
	/// Proof: `XcmPallet::VersionNotifiers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_version_notifiers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `233`
		//  Estimated: `11123`
		// Minimum execution time: 13_085_000 picoseconds.
		Weight::from_parts(13_365_000, 0)
			.saturating_add(Weight::from_parts(0, 11123))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `XcmPallet::VersionNotifyTargets` (r:5 w:0)
	/// Proof: `XcmPallet::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn already_notified_target() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `243`
		//  Estimated: `13608`
		// Minimum execution time: 14_858_000 picoseconds.
		Weight::from_parts(15_148_000, 0)
			.saturating_add(Weight::from_parts(0, 13608))
			.saturating_add(T::DbWeight::get().reads(5))
	}
	/// Storage: `XcmPallet::VersionNotifyTargets` (r:2 w:1)
	/// Proof: `XcmPallet::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `XcmPallet::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SafeXcmVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn notify_current_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `597`
		//  Estimated: `6537`
		// Minimum execution time: 29_236_000 picoseconds.
		Weight::from_parts(30_077_000, 0)
			.saturating_add(Weight::from_parts(0, 6537))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `XcmPallet::VersionNotifyTargets` (r:3 w:0)
	/// Proof: `XcmPallet::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn notify_target_migration_fail() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `272`
		//  Estimated: `8687`
		// Minimum execution time: 7_725_000 picoseconds.
		Weight::from_parts(7_836_000, 0)
			.saturating_add(Weight::from_parts(0, 8687))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: `XcmPallet::VersionNotifyTargets` (r:4 w:2)
	/// Proof: `XcmPallet::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_version_notify_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `11130`
		// Minimum execution time: 13_395_000 picoseconds.
		Weight::from_parts(13_605_000, 0)
			.saturating_add(Weight::from_parts(0, 11130))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `XcmPallet::VersionNotifyTargets` (r:4 w:2)
	/// Proof: `XcmPallet::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Configuration::ActiveConfig` (r:1 w:0)
	/// Proof: `Configuration::ActiveConfig` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::VersionDiscoveryQueue` (r:1 w:1)
	/// Proof: `XcmPallet::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SafeXcmVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn migrate_and_notify_old_targets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `11491`
		// Minimum execution time: 34_765_000 picoseconds.
		Weight::from_parts(35_616_000, 0)
			.saturating_add(Weight::from_parts(0, 11491))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
