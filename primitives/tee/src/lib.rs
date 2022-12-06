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

//!Primitives for TEE
#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use ias_verify::SgxBuildMode;
use scale_info::TypeInfo;
use sp_std::prelude::*;

/// Struct for holding Enclave Information.
#[derive(Encode, Decode, Default, Copy, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct Enclave<PubKey, Url, ShieldingKey> {
	/// The pubkey is the account ID for the enclave.
	pub pubkey: PubKey,
	/// mrenclave
	pub mr_enclave: [u8; 32],
	/// The shielding public key of the enclave, which is used by the client for encryption
	pub shielding_key: ShieldingKey,
	// TODO: make timestamp: Moment
	/// Enclave update time
	pub timestamp: u64, // unix epoch in milliseconds
	/// Keyholder url
	pub url: Url,       // utf8 encoded url
	/// Enclave build mode
	pub sgx_mode: SgxBuildMode,
}

/// Impl for holding Enclave Information.
impl<PubKey, Url,ShieldingKey> Enclave<PubKey, Url, ShieldingKey> {
	/// new method
	pub fn new(
		pubkey: PubKey,
		mr_enclave: [u8; 32],
		shielding_key: ShieldingKey,
		timestamp: u64,
		url: Url,
		sgx_build_mode: SgxBuildMode,
	) -> Self {
		Enclave { pubkey, mr_enclave, shielding_key, timestamp, url, sgx_mode: sgx_build_mode }
	}
}
