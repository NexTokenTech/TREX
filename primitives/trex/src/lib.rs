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
use scale_info::TypeInfo;
use sp_core::offchain::Timestamp;
use sp_std::prelude::*;
use sp_core::RuntimeDebug;

pub type ShieldedKey = Vec<u8>;

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
pub struct TREXData<AccountID,Moment> {
	pub cipher: Vec<u8>,
	pub from: AccountID,
	pub release_time: Moment,
	/// Each key piece contains a share of secret key and its destination node ID.
	pub key_pieces: Vec<u8>,
}