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

//! Autogenerated weights for joystream_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-06-27, STEPS: `1`, REPEAT: 1, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=joystream_utility
// --extrinsic=*
// --chain=dev
// --steps=1
// --repeat=1
// --execution=wasm
// --no-verify
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for joystream_utility.
pub trait WeightInfo {
	fn execute_signal_proposal(i: u32, ) -> Weight;
	fn update_working_group_budget_positive() -> Weight;
	fn update_working_group_budget_negative() -> Weight;
	fn burn_account_tokens() -> Weight;
}

/// Weights for joystream_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn execute_signal_proposal(_i: u32, ) -> Weight {
		(43_000_000 as Weight)
	}
	// Storage: unknown [0xcf9da36cc34d922a84a3ec231495ea2bf3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn update_working_group_budget_positive() -> Weight {
		(19_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0xcf9da36cc34d922a84a3ec231495ea2bf3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn update_working_group_budget_negative() -> Weight {
		(18_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn burn_account_tokens() -> Weight {
		(27_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn execute_signal_proposal(_i: u32, ) -> Weight {
		0
	}
	fn update_working_group_budget_positive() -> Weight {
		0
	}
	fn update_working_group_budget_negative() -> Weight {
		0
	}
	fn burn_account_tokens() -> Weight {
		0
	}
}
