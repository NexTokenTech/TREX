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
