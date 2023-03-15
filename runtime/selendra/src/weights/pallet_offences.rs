
//! Autogenerated weights for `pallet_offences`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-03, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `benchmarking`, CPU: `digital-ocean`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=pallet_offences
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/pallet_offences.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_offences`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_offences::WeightInfo for WeightInfo<T> {
	// Storage: Offences ReportsByKindIndex (r:1 w:1)
	// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	// Storage: Offences Reports (r:100 w:100)
	// Storage: Staking SlashRewardFraction (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Staking Invulnerables (r:1 w:0)
	// Storage: Staking ValidatorSlashInEra (r:100 w:100)
	// Storage: Staking SlashingSpans (r:1700 w:1700)
	// Storage: Staking SpanSlash (r:1700 w:1700)
	// Storage: Staking Validators (r:100 w:100)
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: VoterList ListNodes (r:299 w:299)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking Nominators (r:100 w:0)
	// Storage: Staking OffendingValidators (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Staking NominatorSlashInEra (r:1600 w:1600)
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	/// The range of component `o` is `[2, 100]`.
	/// The range of component `n` is `[0, 16]`.
	fn report_offence_im_online(r: u32, o: u32, n: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 10_576_000
			.saturating_add(Weight::from_ref_time(134_433_000 as u64).saturating_mul(r as u64))
			// Standard Error: 10_712_000
			.saturating_add(Weight::from_ref_time(1_104_207_000 as u64).saturating_mul(o as u64))
			// Standard Error: 62_863_000
			.saturating_add(Weight::from_ref_time(3_290_372_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads((57 as u64).saturating_mul(o as u64)))
			.saturating_add(T::DbWeight::get().reads((305 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((56 as u64).saturating_mul(o as u64)))
			.saturating_add(T::DbWeight::get().writes((305 as u64).saturating_mul(n as u64)))
	}
	// Storage: Offences ReportsByKindIndex (r:1 w:1)
	// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	// Storage: Offences Reports (r:1 w:1)
	// Storage: Staking SlashRewardFraction (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Staking Invulnerables (r:1 w:0)
	// Storage: Staking ValidatorSlashInEra (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking SpanSlash (r:1 w:1)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking OffendingValidators (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	// Storage: Staking NominatorSlashInEra (r:1 w:1)
	/// The range of component `n` is `[0, 16]`.
	fn report_offence_grandpa(n: u32, ) -> Weight {
		Weight::from_ref_time(184_989_000 as u64)
			// Standard Error: 163_000
			.saturating_add(Weight::from_ref_time(16_577_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(20 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
	// Storage: Offences ReportsByKindIndex (r:1 w:1)
	// Storage: Offences ConcurrentReportsIndex (r:1 w:1)
	// Storage: Offences Reports (r:1 w:1)
	// Storage: Staking SlashRewardFraction (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasStartSessionIndex (r:1 w:0)
	// Storage: Staking Invulnerables (r:1 w:0)
	// Storage: Staking ValidatorSlashInEra (r:1 w:1)
	// Storage: Staking SlashingSpans (r:1 w:1)
	// Storage: Staking SpanSlash (r:1 w:1)
	// Storage: Staking Validators (r:1 w:1)
	// Storage: Staking CounterForValidators (r:1 w:1)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: Staking OffendingValidators (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: Staking UnappliedSlashes (r:1 w:1)
	// Storage: Staking NominatorSlashInEra (r:1 w:1)
	/// The range of component `n` is `[0, 16]`.
	fn report_offence_babe(n: u32, ) -> Weight {
		Weight::from_ref_time(177_464_000 as u64)
			// Standard Error: 141_000
			.saturating_add(Weight::from_ref_time(17_254_000 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(20 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
}