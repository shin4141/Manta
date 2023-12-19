// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_asset_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("manta-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=40
// --pallet=pallet_asset_manager
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/home/jamie/my-repo/Manta/runtime/calamari/src/weights/pallet_asset_manager.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_asset_manager.
pub trait WeightInfo {
	fn register_asset() -> Weight;
	fn set_units_per_second() -> Weight;
	fn update_asset_location() -> Weight;
	fn update_asset_metadata() -> Weight;
	fn mint_asset() -> Weight;
	fn set_min_xcm_fee() -> Weight;
	fn update_outgoing_filtered_assets() -> Weight;
	fn register_lp_asset() -> Weight;
	fn permissionless_register_asset() -> Weight;
}

/// Weights for pallet_asset_manager using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: AssetManager LocationAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager LocationAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager NextAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdLocation (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	fn register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `250`
		//  Estimated: `3715`
		// Minimum execution time: 18_374_000 picoseconds.
		Weight::from_parts(18_745_000, 3715)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager UnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager UnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	fn set_units_per_second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1443`
		//  Estimated: `4908`
		// Minimum execution time: 18_254_000 picoseconds.
		Weight::from_parts(22_352_000, 4908)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager LocationAssetId (r:1 w:2)
	/// Proof Skipped: AssetManager LocationAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AllowedDestParaIds (r:2 w:2)
	/// Proof Skipped: AssetManager AllowedDestParaIds (max_values: None, max_size: None, mode: Measured)
	fn update_asset_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4098`
		//  Estimated: `10038`
		// Minimum execution time: 34_525_000 picoseconds.
		Weight::from_parts(38_853_000, 10038)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	fn update_asset_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4077`
		//  Estimated: `7542`
		// Minimum execution time: 35_798_000 picoseconds.
		Weight::from_parts(44_854_000, 7542)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn mint_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3496`
		//  Estimated: `6961`
		// Minimum execution time: 101_220_000 picoseconds.
		Weight::from_parts(122_901_000, 6961)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: AssetManager MinXcmFee (r:0 w:1)
	/// Proof Skipped: AssetManager MinXcmFee (max_values: None, max_size: None, mode: Measured)
	fn set_min_xcm_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_062_000 picoseconds.
		Weight::from_parts(14_107_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: AssetManager FilteredOutgoingAssetLocations (r:0 w:1)
	/// Proof Skipped: AssetManager FilteredOutgoingAssetLocations (max_values: None, max_size: None, mode: Measured)
	fn update_outgoing_filtered_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_922_000 picoseconds.
		Weight::from_parts(16_441_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:2 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdPairToLp (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdPairToLp (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager NextAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager LpToAssetIdPair (r:0 w:1)
	/// Proof Skipped: AssetManager LpToAssetIdPair (max_values: None, max_size: None, mode: Measured)
	fn register_lp_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `545`
		//  Estimated: `6485`
		// Minimum execution time: 25_388_000 picoseconds.
		Weight::from_parts(26_420_000, 6485)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: AssetManager NextPermissionlessAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager NextPermissionlessAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	fn permissionless_register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3687`
		// Minimum execution time: 105_138_000 picoseconds.
		Weight::from_parts(117_591_000, 3687)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: AssetManager LocationAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager LocationAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager NextAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdLocation (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	fn register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `250`
		//  Estimated: `3715`
		// Minimum execution time: 18_374_000 picoseconds.
		Weight::from_parts(18_745_000, 3715)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager UnitsPerSecond (r:0 w:1)
	/// Proof Skipped: AssetManager UnitsPerSecond (max_values: None, max_size: None, mode: Measured)
	fn set_units_per_second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1443`
		//  Estimated: `4908`
		// Minimum execution time: 18_254_000 picoseconds.
		Weight::from_parts(22_352_000, 4908)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager LocationAssetId (r:1 w:2)
	/// Proof Skipped: AssetManager LocationAssetId (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AllowedDestParaIds (r:2 w:2)
	/// Proof Skipped: AssetManager AllowedDestParaIds (max_values: None, max_size: None, mode: Measured)
	fn update_asset_location() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4098`
		//  Estimated: `10038`
		// Minimum execution time: 34_525_000 picoseconds.
		Weight::from_parts(38_853_000, 10038)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:0)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	fn update_asset_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4077`
		//  Estimated: `7542`
		// Minimum execution time: 35_798_000 picoseconds.
		Weight::from_parts(44_854_000, 7542)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:1 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	fn mint_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3496`
		//  Estimated: `6961`
		// Minimum execution time: 101_220_000 picoseconds.
		Weight::from_parts(122_901_000, 6961)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: AssetManager MinXcmFee (r:0 w:1)
	/// Proof Skipped: AssetManager MinXcmFee (max_values: None, max_size: None, mode: Measured)
	fn set_min_xcm_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_062_000 picoseconds.
		Weight::from_parts(14_107_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: AssetManager FilteredOutgoingAssetLocations (r:0 w:1)
	/// Proof Skipped: AssetManager FilteredOutgoingAssetLocations (max_values: None, max_size: None, mode: Measured)
	fn update_outgoing_filtered_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_922_000 picoseconds.
		Weight::from_parts(16_441_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: AssetManager AssetIdLocation (r:2 w:0)
	/// Proof Skipped: AssetManager AssetIdLocation (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager AssetIdPairToLp (r:1 w:1)
	/// Proof Skipped: AssetManager AssetIdPairToLp (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager NextAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: AssetManager LpToAssetIdPair (r:0 w:1)
	/// Proof Skipped: AssetManager LpToAssetIdPair (max_values: None, max_size: None, mode: Measured)
	fn register_lp_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `545`
		//  Estimated: `6485`
		// Minimum execution time: 25_388_000 picoseconds.
		Weight::from_parts(26_420_000, 6485)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: AssetManager NextPermissionlessAssetId (r:1 w:1)
	/// Proof Skipped: AssetManager NextPermissionlessAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets Asset (r:1 w:1)
	/// Proof: Assets Asset (max_values: None, max_size: Some(222), added: 2697, mode: MaxEncodedLen)
	/// Storage: Assets Metadata (r:1 w:1)
	/// Proof: Assets Metadata (max_values: None, max_size: Some(152), added: 2627, mode: MaxEncodedLen)
	/// Storage: Assets Account (r:1 w:1)
	/// Proof: Assets Account (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	/// Storage: AssetManager AssetIdMetadata (r:0 w:1)
	/// Proof Skipped: AssetManager AssetIdMetadata (max_values: None, max_size: None, mode: Measured)
	fn permissionless_register_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221`
		//  Estimated: `3687`
		// Minimum execution time: 105_138_000 picoseconds.
		Weight::from_parts(117_591_000, 3687)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
}
