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

//! Autogenerated weights for storage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-13, STEPS: `10`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=storage
// --extrinsic=*
// --chain=dev
// --steps=10
// --repeat=5
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for storage.
pub trait WeightInfo {
	fn delete_storage_bucket() -> Weight;
	fn update_uploading_blocked_status() -> Weight;
	fn update_data_size_fee() -> Weight;
	fn update_storage_buckets_per_bag_limit() -> Weight;
	fn update_storage_buckets_voucher_max_limits() -> Weight;
	fn update_data_object_state_bloat_bond() -> Weight;
	fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy() -> Weight;
	fn update_blacklist(_i: u32, _j: u32, ) -> Weight;
	fn create_storage_bucket() -> Weight;
	fn update_storage_buckets_for_bag(_i: u32, _j: u32, ) -> Weight;
	fn cancel_storage_bucket_operator_invite() -> Weight;
	fn invite_storage_bucket_operator() -> Weight;
	fn remove_storage_bucket_operator() -> Weight;
	fn update_storage_bucket_status() -> Weight;
	fn set_storage_bucket_voucher_limits() -> Weight;
	fn accept_storage_bucket_invitation() -> Weight;
	fn set_storage_operator_metadata(_i: u32, ) -> Weight;
	fn accept_pending_data_objects(_i: u32, ) -> Weight;
	fn create_distribution_bucket_family() -> Weight;
	fn delete_distribution_bucket_family() -> Weight;
	fn create_distribution_bucket() -> Weight;
	fn update_distribution_bucket_status() -> Weight;
	fn delete_distribution_bucket() -> Weight;
	fn update_distribution_buckets_for_bag(_i: u32, _j: u32, ) -> Weight;
	fn update_distribution_buckets_per_bag_limit() -> Weight;
	fn update_distribution_bucket_mode() -> Weight;
	fn update_families_in_dynamic_bag_creation_policy(_i: u32, ) -> Weight;
	fn invite_distribution_bucket_operator() -> Weight;
	fn cancel_distribution_bucket_operator_invite() -> Weight;
	fn remove_distribution_bucket_operator() -> Weight;
	fn set_distribution_bucket_family_metadata(_i: u32, ) -> Weight;
	fn accept_distribution_bucket_invitation() -> Weight;
	fn set_distribution_operator_metadata(_i: u32, ) -> Weight;
	fn storage_operator_remark(_i: u32, ) -> Weight;
	fn distribution_operator_remark(_i: u32, ) -> Weight;
}

/// Weights for storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn delete_storage_bucket() -> Weight {
		(33_588_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage UploadingBlocked (r:0 w:1)
	fn update_uploading_blocked_status() -> Weight {
		(27_171_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DataObjectPerMegabyteFee (r:0 w:1)
	fn update_data_size_fee() -> Weight {
		(28_654_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketsPerBagLimit (r:0 w:1)
	fn update_storage_buckets_per_bag_limit() -> Weight {
		(26_878_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsSizeLimit (r:0 w:1)
	// Storage: Storage VoucherMaxObjectsNumberLimit (r:0 w:1)
	fn update_storage_buckets_voucher_max_limits() -> Weight {
		(45_276_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DataObjectStateBloatBondValue (r:0 w:1)
	fn update_data_object_state_bloat_bond() -> Weight {
		(27_263_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DynamicBagCreationPolicies (r:1 w:1)
	fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy() -> Weight {
		(31_642_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage Blacklist (r:10000 w:0)
	// Storage: Storage CurrentBlacklistSize (r:1 w:1)
	fn update_blacklist(i: u32, j: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 47_000
			.saturating_add((5_297_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 47_000
			.saturating_add((3_400_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsSizeLimit (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsNumberLimit (r:1 w:0)
	// Storage: Storage NextStorageBucketId (r:1 w:1)
	// Storage: Storage StorageBucketById (r:0 w:1)
	fn create_storage_bucket() -> Weight {
		(36_324_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage Bags (r:1 w:1)
	// Storage: Storage StorageBucketsPerBagLimit (r:1 w:0)
	// Storage: Storage StorageBucketById (r:21 w:21)
	fn update_storage_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		(60_824_000 as Weight)
			// Standard Error: 226_000
			.saturating_add((17_026_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 226_000
			.saturating_add((15_440_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(j as Weight)))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn cancel_storage_bucket_operator_invite() -> Weight {
		(35_665_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:2 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn invite_storage_bucket_operator() -> Weight {
		(40_267_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn remove_storage_bucket_operator() -> Weight {
		(34_897_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn update_storage_bucket_status() -> Weight {
		(33_559_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	// Storage: Storage VoucherMaxObjectsSizeLimit (r:1 w:0)
	// Storage: Storage VoucherMaxObjectsNumberLimit (r:1 w:0)
	fn set_storage_bucket_voucher_limits() -> Weight {
		(39_322_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:1)
	fn accept_storage_bucket_invitation() -> Weight {
		(32_841_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:0)
	fn set_storage_operator_metadata(i: u32, ) -> Weight {
		(30_193_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Storage StorageBucketById (r:1 w:0)
	// Storage: Storage Bags (r:1 w:0)
	// Storage: Storage DataObjectsById (r:1 w:1)
	fn accept_pending_data_objects(i: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 89_000
			.saturating_add((11_209_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyNumber (r:1 w:1)
	// Storage: Storage NextDistributionBucketFamilyId (r:1 w:1)
	// Storage: Storage DistributionBucketFamilyById (r:0 w:1)
	fn create_distribution_bucket_family() -> Weight {
		(36_876_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:1)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:0)
	// Storage: Storage DynamicBagCreationPolicies (r:2 w:0)
	// Storage: Storage DistributionBucketFamilyNumber (r:1 w:1)
	fn delete_distribution_bucket_family() -> Weight {
		(52_121_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:1)
	// Storage: Storage DistributionBucketByFamilyIdById (r:0 w:1)
	fn create_distribution_bucket() -> Weight {
		(38_099_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn update_distribution_bucket_status() -> Weight {
		(37_614_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn delete_distribution_bucket() -> Weight {
		(36_772_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage Bags (r:1 w:1)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:0)
	// Storage: Storage DistributionBucketsPerBagLimit (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:101 w:101)
	fn update_distribution_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		(3_396_000 as Weight)
			// Standard Error: 145_000
			.saturating_add((11_936_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 145_000
			.saturating_add((12_232_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(j as Weight)))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketsPerBagLimit (r:0 w:1)
	fn update_distribution_buckets_per_bag_limit() -> Weight {
		(27_284_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn update_distribution_bucket_mode() -> Weight {
		(37_323_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:2 w:0)
	// Storage: Storage DynamicBagCreationPolicies (r:1 w:1)
	fn update_families_in_dynamic_bag_creation_policy(i: u32, ) -> Weight {
		(35_179_000 as Weight)
			// Standard Error: 220_000
			.saturating_add((5_222_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:2 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn invite_distribution_bucket_operator() -> Weight {
		(43_655_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn cancel_distribution_bucket_operator_invite() -> Weight {
		(39_873_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn remove_distribution_bucket_operator() -> Weight {
		(39_790_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup CurrentLead (r:1 w:0)
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketFamilyById (r:1 w:0)
	fn set_distribution_bucket_family_metadata(i: u32, ) -> Weight {
		(33_314_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
	}
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:1)
	fn accept_distribution_bucket_invitation() -> Weight {
		(37_473_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:0)
	fn set_distribution_operator_metadata(i: u32, ) -> Weight {
		(34_448_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Instance2WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage StorageBucketById (r:1 w:0)
	fn storage_operator_remark(i: u32, ) -> Weight {
		(30_380_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Instance9WorkingGroup WorkerById (r:1 w:0)
	// Storage: Storage DistributionBucketByFamilyIdById (r:1 w:0)
	fn distribution_operator_remark(i: u32, ) -> Weight {
		(34_309_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn delete_storage_bucket() -> Weight {
		0
	}
	fn update_uploading_blocked_status() -> Weight {
		0
	}
	fn update_data_size_fee() -> Weight {
		0
	}
	fn update_storage_buckets_per_bag_limit() -> Weight {
		0
	}
	fn update_storage_buckets_voucher_max_limits() -> Weight {
		0
	}
	fn update_data_object_state_bloat_bond() -> Weight {
		0
	}
	fn update_number_of_storage_buckets_in_dynamic_bag_creation_policy() -> Weight {
		0
	}
	fn update_blacklist(i: u32, j: u32, ) -> Weight {
		0
	}
	fn create_storage_bucket() -> Weight {
		0
	}
	fn update_storage_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		0
	}
	fn cancel_storage_bucket_operator_invite() -> Weight {
		0
	}
	fn invite_storage_bucket_operator() -> Weight {
		0
	}
	fn remove_storage_bucket_operator() -> Weight {
		0
	}
	fn update_storage_bucket_status() -> Weight {
		0
	}
	fn set_storage_bucket_voucher_limits() -> Weight {
		0
	}
	fn accept_storage_bucket_invitation() -> Weight {
		0
	}
	fn set_storage_operator_metadata(i: u32, ) -> Weight {
		0
	}
	fn accept_pending_data_objects(i: u32, ) -> Weight {
		0
	}
	fn create_distribution_bucket_family() -> Weight {
		0
	}
	fn delete_distribution_bucket_family() -> Weight {
		0
	}
	fn create_distribution_bucket() -> Weight {
		0
	}
	fn update_distribution_bucket_status() -> Weight {
		0
	}
	fn delete_distribution_bucket() -> Weight {
		0
	}
	fn update_distribution_buckets_for_bag(i: u32, j: u32, ) -> Weight {
		0
	}
	fn update_distribution_buckets_per_bag_limit() -> Weight {
		0
	}
	fn update_distribution_bucket_mode() -> Weight {
		0
	}
	fn update_families_in_dynamic_bag_creation_policy(i: u32, ) -> Weight {
		0
	}
	fn invite_distribution_bucket_operator() -> Weight {
		0
	}
	fn cancel_distribution_bucket_operator_invite() -> Weight {
		0
	}
	fn remove_distribution_bucket_operator() -> Weight {
		0
	}
	fn set_distribution_bucket_family_metadata(i: u32, ) -> Weight {
		0
	}
	fn accept_distribution_bucket_invitation() -> Weight {
		0
	}
	fn set_distribution_operator_metadata(i: u32, ) -> Weight {
		0
	}
	fn storage_operator_remark(i: u32, ) -> Weight {
		0
	}
	fn distribution_operator_remark(i: u32, ) -> Weight {
		0
	}
}
