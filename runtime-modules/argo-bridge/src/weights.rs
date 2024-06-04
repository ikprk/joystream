// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for argo_bridge
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-05-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("prod-test"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/debug/joystream-node
// benchmark
// pallet
// --pallet=argo-bridge
// --extrinsic=*
// --chain=prod-test
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=./scripts/../runtime-modules/argo-bridge/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for argo_bridge.
pub trait WeightInfo {
	fn request_outbound_transfer() -> Weight;
	fn finalize_inbound_transfer() -> Weight;
	fn pause_bridge() -> Weight;
	fn init_unpause_bridge() -> Weight;
	fn finish_unpause_bridge() -> Weight;
	fn update_bridge_constrains() -> Weight;
}

/// Weights for argo_bridge using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ArgoBridge Status (r:1 w:0)
	// Proof: ArgoBridge Status (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: ArgoBridge RemoteChains (r:1 w:0)
	// Proof: ArgoBridge RemoteChains (max_values: Some(1), max_size: Some(41), added: 536, mode: MaxEncodedLen)
	// Storage: ArgoBridge BridgingFee (r:1 w:0)
	// Proof: ArgoBridge BridgingFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: ArgoBridge MintAllowance (r:1 w:1)
	// Proof: ArgoBridge MintAllowance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: ArgoBridge NextTransferId (r:1 w:1)
	// Proof: ArgoBridge NextTransferId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn request_outbound_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `11104`
		// Minimum execution time: 998_107 nanoseconds.
		Weight::from_parts(1_038_838_000, 0u64)
			.saturating_add(Weight::from_parts(0, 11104))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: ArgoBridge OperatorAccount (r:1 w:0)
	// Proof: ArgoBridge OperatorAccount (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: ArgoBridge Status (r:1 w:0)
	// Proof: ArgoBridge Status (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	// Storage: ArgoBridge MintAllowance (r:1 w:1)
	// Proof: ArgoBridge MintAllowance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: ArgoBridge RemoteChains (r:1 w:0)
	// Proof: ArgoBridge RemoteChains (max_values: Some(1), max_size: Some(41), added: 536, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn finalize_inbound_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `473`
		//  Estimated: `9627`
		// Minimum execution time: 883_138 nanoseconds.
		Weight::from_parts(900_668_000, 0u64)
			.saturating_add(Weight::from_parts(0, 9627))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: ArgoBridge PauserAccounts (r:1 w:0)
	// Proof: ArgoBridge PauserAccounts (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	// Storage: ArgoBridge Status (r:0 w:1)
	// Proof: ArgoBridge Status (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	fn pause_bridge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `498`
		//  Estimated: `1806`
		// Minimum execution time: 371_039 nanoseconds.
		Weight::from_parts(382_129_000, 0u64)
			.saturating_add(Weight::from_parts(0, 1806))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ArgoBridge PauserAccounts (r:1 w:0)
	// Proof: ArgoBridge PauserAccounts (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	// Storage: ArgoBridge ThawnDuration (r:1 w:0)
	// Proof: ArgoBridge ThawnDuration (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: ArgoBridge Status (r:0 w:1)
	// Proof: ArgoBridge Status (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	fn init_unpause_bridge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `498`
		//  Estimated: `3295`
		// Minimum execution time: 402_659 nanoseconds.
		Weight::from_parts(420_319_000, 0u64)
			.saturating_add(Weight::from_parts(0, 3295))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ArgoBridge OperatorAccount (r:1 w:0)
	// Proof: ArgoBridge OperatorAccount (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: ArgoBridge Status (r:1 w:1)
	// Proof: ArgoBridge Status (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	fn finish_unpause_bridge() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `173`
		//  Estimated: `3007`
		// Minimum execution time: 412_269 nanoseconds.
		Weight::from_parts(422_459_000, 0u64)
			.saturating_add(Weight::from_parts(0, 3007))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: ArgoBridge RemoteChains (r:0 w:1)
	// Proof: ArgoBridge RemoteChains (max_values: Some(1), max_size: Some(41), added: 536, mode: MaxEncodedLen)
	// Storage: ArgoBridge PauserAccounts (r:0 w:1)
	// Proof: ArgoBridge PauserAccounts (max_values: Some(1), max_size: Some(321), added: 816, mode: MaxEncodedLen)
	// Storage: ArgoBridge OperatorAccount (r:0 w:1)
	// Proof: ArgoBridge OperatorAccount (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	// Storage: ArgoBridge BridgingFee (r:0 w:1)
	// Proof: ArgoBridge BridgingFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: ArgoBridge ThawnDuration (r:0 w:1)
	// Proof: ArgoBridge ThawnDuration (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn update_bridge_constrains() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 293_519 nanoseconds.
		Weight::from_parts(298_009_000, 0u64)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn request_outbound_transfer() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn finalize_inbound_transfer() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn pause_bridge() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn init_unpause_bridge() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn finish_unpause_bridge() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn update_bridge_constrains() -> Weight {
		Weight::from_parts(0, 0)
	}
}
