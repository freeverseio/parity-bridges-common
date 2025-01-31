// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_bridge_parachains
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `serban-ROG-Zephyrus`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/millau-bridge-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_parachains
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/parachains/src/weights.rs
// --template=./.maintain/bridge-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bridge_parachains.
pub trait WeightInfo {
	fn submit_parachain_heads_with_n_parachains(p: u32) -> Weight;
	fn submit_parachain_heads_with_1kb_proof() -> Weight;
	fn submit_parachain_heads_with_16kb_proof() -> Weight;
}

/// Weights for `pallet_bridge_parachains` that are generated using one of the Bridge testnets.
///
/// Those weights are test only and must never be used in production.
pub struct BridgeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for BridgeWeight<T> {
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(1024), max_size:
	/// Some(64), added: 1549, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(1024), max_size:
	/// Some(196), added: 1681, mode: MaxEncodedLen)
	///
	/// The range of component `p` is `[1, 2]`.
	fn submit_parachain_heads_with_n_parachains(_p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3038`
		// Minimum execution time: 30_211 nanoseconds.
		Weight::from_parts(32_633_893, 3038)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(1024), max_size:
	/// Some(64), added: 1549, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(1024), max_size:
	/// Some(196), added: 1681, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_1kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3038`
		// Minimum execution time: 30_830 nanoseconds.
		Weight::from_parts(31_801_000, 3038)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(1024), max_size:
	/// Some(64), added: 1549, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(1024), max_size:
	/// Some(196), added: 1681, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_16kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3038`
		// Minimum execution time: 44_736 nanoseconds.
		Weight::from_parts(45_296_000, 3038)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(1024), max_size:
	/// Some(64), added: 1549, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(1024), max_size:
	/// Some(196), added: 1681, mode: MaxEncodedLen)
	///
	/// The range of component `p` is `[1, 2]`.
	fn submit_parachain_heads_with_n_parachains(_p: u32) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3038`
		// Minimum execution time: 30_211 nanoseconds.
		Weight::from_parts(32_633_893, 3038)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(1024), max_size:
	/// Some(64), added: 1549, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(1024), max_size:
	/// Some(196), added: 1681, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_1kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3038`
		// Minimum execution time: 30_830 nanoseconds.
		Weight::from_parts(31_801_000, 3038)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: BridgeRialtoParachains PalletOperatingMode (r:1 w:0)
	///
	/// Proof: BridgeRialtoParachains PalletOperatingMode (max_values: Some(1), max_size: Some(1),
	/// added: 496, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoGrandpa ImportedHeaders (r:1 w:0)
	///
	/// Proof: BridgeRialtoGrandpa ImportedHeaders (max_values: Some(14400), max_size: Some(68),
	/// added: 2048, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ParasInfo (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ParasInfo (max_values: Some(1), max_size: Some(60), added:
	/// 555, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHashes (r:1 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHashes (max_values: Some(1024), max_size:
	/// Some(64), added: 1549, mode: MaxEncodedLen)
	///
	/// Storage: BridgeRialtoParachains ImportedParaHeads (r:0 w:1)
	///
	/// Proof: BridgeRialtoParachains ImportedParaHeads (max_values: Some(1024), max_size:
	/// Some(196), added: 1681, mode: MaxEncodedLen)
	fn submit_parachain_heads_with_16kb_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `3038`
		// Minimum execution time: 44_736 nanoseconds.
		Weight::from_parts(45_296_000, 3038)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
