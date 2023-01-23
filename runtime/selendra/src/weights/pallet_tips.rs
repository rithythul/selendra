
//! Autogenerated weights for `pallet_tips`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `selendra-benchamarking`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=pallet_tips
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/pallet_tips.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `r` is `[0, 16384]`.
	fn report_awesome(r: u32, ) -> Weight {
		Weight::from_ref_time(29_098_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		Weight::from_ref_time(27_830_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	/// The range of component `r` is `[0, 16384]`.
	/// The range of component `t` is `[1, 13]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		Weight::from_ref_time(20_185_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(r as u64))
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(204_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	/// The range of component `t` is `[1, 13]`.
	fn tip(t: u32, ) -> Weight {
		Weight::from_ref_time(14_650_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(169_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 13]`.
	fn close_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(44_468_000 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(186_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	/// The range of component `t` is `[1, 13]`.
	fn slash_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(18_758_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(34_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
