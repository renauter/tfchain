
//! Autogenerated weights for pallet_validator
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-02, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ragnar`, CPU: `Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../target/release/tfchain
// benchmark
// pallet
// --chain=dev
// --pallet=pallet_validator
// --extrinsic=*
// --steps=50
// --repeat=20
// --execution=wasm
// --heap-pages=409
// --output
// ../pallets/pallet-validator/src/weights.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_validator.
pub trait WeightInfo {
	fn create_validator_request() -> Weight;
	fn activate_validator_node() -> Weight;
	fn change_validator_node_account() -> Weight;
	fn bond() -> Weight;
	fn approve_validator() -> Weight;
	fn remove_validator() -> Weight;
}

/// Weights for pallet_validator using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	fn create_validator_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 20_271_000 picoseconds.
		Weight::from_parts(20_728_000, 3507)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: ValidatorSet Validators (r:1 w:1)
	/// Proof Skipped: ValidatorSet Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ValidatorSet ApprovedValidators (r:1 w:1)
	/// Proof Skipped: ValidatorSet ApprovedValidators (max_values: Some(1), max_size: None, mode: Measured)
	fn activate_validator_node() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `365`
		//  Estimated: `3830`
		// Minimum execution time: 45_601_000 picoseconds.
		Weight::from_parts(46_151_000, 3830)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: ValidatorSet Validators (r:1 w:1)
	/// Proof Skipped: ValidatorSet Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ValidatorSet ApprovedValidators (r:1 w:1)
	/// Proof Skipped: ValidatorSet ApprovedValidators (max_values: Some(1), max_size: None, mode: Measured)
	fn change_validator_node_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431`
		//  Estimated: `3896`
		// Minimum execution time: 61_641_000 picoseconds.
		Weight::from_parts(62_171_000, 3896)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Validator Bonded (r:1 w:1)
	/// Proof Skipped: Validator Bonded (max_values: None, max_size: None, mode: Measured)
	fn bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 17_991_000 picoseconds.
		Weight::from_parts(18_269_000, 3507)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn approve_validator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558`
		//  Estimated: `4687`
		// Minimum execution time: 46_008_000 picoseconds.
		Weight::from_parts(46_565_000, 4687)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CouncilMembership Prime (r:1 w:0)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_validator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622`
		//  Estimated: `4687`
		// Minimum execution time: 39_718_000 picoseconds.
		Weight::from_parts(40_453_000, 4687)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	fn create_validator_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 20_271_000 picoseconds.
		Weight::from_parts(20_728_000, 3507)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: ValidatorSet Validators (r:1 w:1)
	/// Proof Skipped: ValidatorSet Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ValidatorSet ApprovedValidators (r:1 w:1)
	/// Proof Skipped: ValidatorSet ApprovedValidators (max_values: Some(1), max_size: None, mode: Measured)
	fn activate_validator_node() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `365`
		//  Estimated: `3830`
		// Minimum execution time: 45_601_000 picoseconds.
		Weight::from_parts(46_151_000, 3830)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: ValidatorSet Validators (r:1 w:1)
	/// Proof Skipped: ValidatorSet Validators (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ValidatorSet ApprovedValidators (r:1 w:1)
	/// Proof Skipped: ValidatorSet ApprovedValidators (max_values: Some(1), max_size: None, mode: Measured)
	fn change_validator_node_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431`
		//  Estimated: `3896`
		// Minimum execution time: 61_641_000 picoseconds.
		Weight::from_parts(62_171_000, 3896)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Validator Bonded (r:1 w:1)
	/// Proof Skipped: Validator Bonded (max_values: None, max_size: None, mode: Measured)
	fn bond() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 17_991_000 picoseconds.
		Weight::from_parts(18_269_000, 3507)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn approve_validator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558`
		//  Estimated: `4687`
		// Minimum execution time: 46_008_000 picoseconds.
		Weight::from_parts(46_565_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: CouncilMembership Members (r:1 w:1)
	/// Proof: CouncilMembership Members (max_values: Some(1), max_size: Some(3202), added: 3697, mode: MaxEncodedLen)
	/// Storage: Validator Validator (r:1 w:1)
	/// Proof Skipped: Validator Validator (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CouncilMembership Prime (r:1 w:0)
	/// Proof: CouncilMembership Prime (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_validator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `622`
		//  Estimated: `4687`
		// Minimum execution time: 39_718_000 picoseconds.
		Weight::from_parts(40_453_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}