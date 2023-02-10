/*
	Copyright 2022 NexToken Tech LLC

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/
#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime::AccountId32;
use sp_std::vec::Vec;
use tee_primitives::Enclave;
// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime/src/lib.rs`)
sp_api::decl_runtime_apis! {
	pub trait TeeApi{
		fn enclave_count() -> u32;
		fn enclave_select(need_count:u64) -> Vec<Enclave<AccountId32,Vec<u8>,Vec<u8>>>;
	}
}
