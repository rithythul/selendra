
//! Autogenerated weights for `frame_benchmarking`
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
// --pallet=frame_benchmarking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/frame_benchmarking.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_benchmarking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_benchmarking::WeightInfo for WeightInfo<T> {
	fn addition(_i: u32, ) -> Weight {
		(1_196_000 as Weight)
	}
	fn subtraction(_i: u32, ) -> Weight {
		(1_015_000 as Weight)
	}
	fn multiplication(_i: u32, ) -> Weight {
		(1_160_000 as Weight)
	}
	fn division(_i: u32, ) -> Weight {
		(1_128_000 as Weight)
	}
	fn hashing(_i: u32, ) -> Weight {
		(17_834_475_000 as Weight)
	}
	fn sr25519_verification(i: u32, ) -> Weight {
		(805_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((45_122_000 as Weight).saturating_mul(i as Weight))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn storage_read(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 6_000
			.saturating_add((1_974_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn storage_write(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 2_000
			.saturating_add((310_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
}
