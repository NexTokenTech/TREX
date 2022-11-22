//! Benchmarking setup for pallet-difficulty

use super::*;

#[allow(unused)]
use crate::Pallet as Trex;
// substrate
use frame_benchmarking::{benchmarks, whitelisted_caller, Vec};
use frame_system::RawOrigin;
// local dependency
use benchmarking_primitives::generate_accounts;
use trex_primitives::{KeyPiece, ShieldedKey, TREXData};

benchmarks! {
	where_clause {  where T::AccountId: From<[u8; 32]> }
	send_trex_data {
		let caller: T::AccountId = whitelisted_caller();
		let cipher = vec![1u8; 100];
		let key: ShieldedKey = vec![1u8; 32];
		let key_piece = KeyPiece{holder: accounts[1].clone(), shielded: key.clone()};
		let key_pieces = vec![key_piece; 8];
		let signed_caller = RawOrigin::Signed(caller);
		// prepare test data
		let test_data = TREXData::<T::AccountId> {
			cipher: cipher.clone(),
			from: accounts[0].clone(),
			key_pieces: key_pieces.clone() };
		let test_byte_data = test_data.encode();
	}: _(signed_caller, cipher.clone(), key_pieces.clone())
	verify {
		assert_eq!(Trex::<T>::trex_storage(), test_byte_data);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
