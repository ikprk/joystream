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

//! Autogenerated weights for content
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=content
// --extrinsic=*
// --chain=dev
// --steps=50
// --repeat=20
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for content.
pub trait WeightInfo {
	fn create_channel(_a: u32, _b: u32, _c: u32, _d: u32, _e: u32, ) -> Weight;
	fn channel_update_with_assets(_a: u32, _b: u32, _c: u32, _d: u32, _e: u32, _f: u32, ) -> Weight;
	fn channel_update_without_assets(_a: u32, _b: u32, _c: u32, _d: u32, ) -> Weight;
	fn delete_channel(_a: u32, _b: u32, _c: u32, ) -> Weight;
	fn issue_creator_token(_a: u32, ) -> Weight;
	fn creator_token_issuer_transfer(_a: u32, _b: u32, ) -> Weight;
	fn make_creator_token_permissionless() -> Weight;
	fn deissue_creator_token() -> Weight;
	fn init_creator_token_sale(_a: u32, ) -> Weight;
}

/// Weights for content using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec4573091994c5737d8f16ba1c53919a94bf2] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87aa6eccf0cc6941ba2e31cdb5870e3229] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b872d56750ffbaedbf3dd8dd3900c756381] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad3323e092df90358439e7c6649f66d93f] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:0 w:10)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:0 w:1)
	fn create_channel(a: u32, b: u32, c: u32, d: u32, e: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 25_534_000
			.saturating_add((149_822_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 12_859_000
			.saturating_add((92_537_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_359_000
			.saturating_add((24_904_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 25_534_000
			.saturating_add((96_091_000 as Weight).saturating_mul(d as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(e as Weight))
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(d as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad235899829563c4064c97520e04fce94e] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad97a953f295d54035e7cdf8d29308e498] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad047d54b603e8604dc1c09c8e0fdc9dc8] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adb7612c99e31defd01cd5a28e9967e208] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:10 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn channel_update_with_assets(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, ) -> Weight {
		(1_094_225_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(d as Weight))
			// Standard Error: 162_000
			.saturating_add((9_528_000 as Weight).saturating_mul(f as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(e as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(f as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(e as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(f as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	fn channel_update_without_assets(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		(61_443_000 as Weight)
			// Standard Error: 1_395_000
			.saturating_add((11_608_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 705_000
			.saturating_add((3_705_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 75_000
			.saturating_add((362_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6add6fd5c94c285d60cbe96c66929f01c31] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adade883233841e9338c8e73f6b9f74890] (r:2 w:1)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad25e0482900c98a1856a1e4878ed6eac6] (r:1 w:0)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6adbaa937139b20b131cc2fcb0072c015f8] (r:20 w:20)
	// Storage: unknown [0x95875cb80ebaf9f918457db6a86ac6ad33807bc23ee2cb31454339d2b2c6b0a8] (r:101 w:101)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn delete_channel(a: u32, b: u32, c: u32, ) -> Weight {
		(475_202_000 as Weight)
			// Standard Error: 611_000
			.saturating_add((4_941_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 316_000
			.saturating_add((6_518_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 36_000
			.saturating_add((9_489_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aede5d7c3c0c0b034c6793a19dcf4ef5c26] (r:1 w:1)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aed5387589e90e72949a247156dadafe9b9] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aed8bdb91db409a5c168f4701a9d4982b38] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedb1a44108963982aa4db86eb1edbfac6e] (r:0 w:1)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedbc811c38c7ece6023fc6c69d9830293d] (r:0 w:1)
	fn issue_creator_token(a: u32, ) -> Weight {
		(294_593_000 as Weight)
			// Standard Error: 10_000
			.saturating_add((6_216_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedbc811c38c7ece6023fc6c69d9830293d] (r:1 w:1)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedb1a44108963982aa4db86eb1edbfac6e] (r:2 w:2)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aed5387589e90e72949a247156dadafe9b9] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn creator_token_issuer_transfer(a: u32, b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 46_000
			.saturating_add((17_959_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(a as Weight)))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedbc811c38c7ece6023fc6c69d9830293d] (r:1 w:1)
	fn make_creator_token_permissionless() -> Weight {
		(280_176_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:1)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedbc811c38c7ece6023fc6c69d9830293d] (r:1 w:1)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aed8bdb91db409a5c168f4701a9d4982b38] (r:0 w:1)
	fn deissue_creator_token() -> Weight {
		(320_165_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b8727cd538b4597adba02e6593ace94e804] (r:1 w:0)
	// Storage: unknown [0x43c6aad67b8d5d8180583e494c8ec457b88c49b6e6ccae735eb57de6295caf6a] (r:1 w:0)
	// Storage: unknown [0xb5a494c92fa4747cc071573e93b32b87af96ab8605611c2e562953c9f5fafe05] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedbc811c38c7ece6023fc6c69d9830293d] (r:1 w:1)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedc4338a04d677e6a769a565096ff20b49] (r:1 w:0)
	// Storage: unknown [0xcb732bb8b688ea549fec1838a1350aedb1a44108963982aa4db86eb1edbfac6e] (r:1 w:1)
	fn init_creator_token_sale(a: u32, ) -> Weight {
		(275_086_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(a as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_channel(a: u32, b: u32, c: u32, d: u32, e: u32, ) -> Weight {
		0
	}
	fn channel_update_with_assets(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, ) -> Weight {
		0
	}
	fn channel_update_without_assets(a: u32, b: u32, c: u32, d: u32, ) -> Weight {
		0
	}
	fn delete_channel(a: u32, b: u32, c: u32, ) -> Weight {
		0
	}
	fn issue_creator_token(a: u32, ) -> Weight {
		0
	}
	fn creator_token_issuer_transfer(a: u32, b: u32, ) -> Weight {
		0
	}
	fn make_creator_token_permissionless() -> Weight {
		0
	}
	fn deissue_creator_token() -> Weight {
		0
	}
	fn init_creator_token_sale(a: u32, ) -> Weight {
		0
	}
}
