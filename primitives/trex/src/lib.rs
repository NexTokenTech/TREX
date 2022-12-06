// Copyright (C) 2022 NexToken Tech LLC.
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

//!Primitives for TREX
#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use frame_system::limits::BlockLength;
use lazy_static::lazy_static;
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_runtime::Perbill;
use sp_std::prelude::*;

/// Enclave encrypted private key
pub type ShieldedKey = Vec<u8>;

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

lazy_static! {
	/// Max size of TREX data in a block (block size).
	pub static ref MAX_TREX_DATA: BlockLength = BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
}

/// Struct for a piece of shielded key.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, TypeInfo)]
pub struct KeyPiece<AccountID> {
	/// This is the account of the key-holder
	pub holder: AccountID,
	/// This is a piece of key shielded by shielding key from a specific key-holder.
	pub shielded: ShieldedKey,
}

/// Struct for holding TREX information.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct TREXData<AccountID, Moment, BlockNumber> {
	/// encrypted message
	pub cipher: Vec<u8>,
	/// The account that sent uxt
	pub from: AccountID,
	/// when to release
	pub release_time: Moment,
	/// The number of the block where uxt is located
	pub current_block: BlockNumber,
	/// Each key piece contains a share of secret key and its destination node ID.
	pub key_pieces: Vec<KeyPiece<AccountID>>,
}

/// Struct for holding TREX Expired Key.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct TREXExpiredKey<AccountID, BlockNumber> {
	/// Expired private key
	pub expired_key: Vec<u8>,
	/// Account source
	pub from: AccountID,
	/// The height of the block pointed to by the private key
	pub block_number: BlockNumber,
	/// The ext index pointed to by the private key
	pub ext_index: BlockNumber,
}
