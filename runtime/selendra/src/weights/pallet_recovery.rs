
//! Autogenerated weights for `pallet_recovery`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=pallet_recovery
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/pallet_recovery.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_recovery`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_recovery::WeightInfo for WeightInfo<T> {
	// Storage: Recovery Proxy (r:1 w:0)
	fn as_recovered() -> Weight {
		(3_911_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Recovery Proxy (r:0 w:1)
	fn set_recovered() -> Weight {
		(9_917_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:1)
	fn create_recovery(n: u32, ) -> Weight {
		(12_568_000 as Weight)
			// Standard Error: 520_000
			.saturating_add((2_663_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn initiate_recovery() -> Weight {
		(21_860_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	fn vouch_recovery(n: u32, ) -> Weight {
		(15_287_000 as Weight)
			// Standard Error: 42_000
			.saturating_add((26_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Recoverable (r:1 w:0)
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Proxy (r:1 w:1)
	fn claim_recovery(n: u32, ) -> Weight {
		(20_174_000 as Weight)
			// Standard Error: 61_000
			.saturating_add((26_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn close_recovery(_n: u32, ) -> Weight {
		(23_864_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Recovery ActiveRecoveries (r:1 w:0)
	// Storage: Recovery Recoverable (r:1 w:1)
	fn remove_recovery(n: u32, ) -> Weight {
		(20_377_000 as Weight)
			// Standard Error: 65_000
			.saturating_add((268_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Recovery Proxy (r:1 w:1)
	fn cancel_recovered() -> Weight {
		(8_102_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
