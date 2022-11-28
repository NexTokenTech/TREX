//! Benchmarking setup for pallet-trex

use super::*;

#[allow(unused)]
use crate::Pallet as Trex;
// substrate
use frame_benchmarking::{
	benchmarks, frame_support::sp_runtime::SaturatedConversion, whitelisted_caller, Vec,
};
use frame_system::RawOrigin;
use test_trex_utils::consts::*;
use trex_primitives::{KeyPiece, ShieldedKey};

benchmarks! {
	where_clause {  where T::AccountId: From<[u8; 32]> }
	send_trex_data {
		let caller: T::AccountId = whitelisted_caller();
		// create cipher text
		let ciphertext = TEST_CIPHER.to_vec();
		// construct key_pieces
		let key: ShieldedKey = SHIELDING_KEY.to_vec();
		let key_piece = KeyPiece { holder: caller.clone(), shielded: key.clone() };
		let key_pieces = vec![key_piece];

		let signed_caller = RawOrigin::Signed(caller.clone());
		let release_time:T::Moment = TEST_TIMESTAMP.saturated_into();
		// prepare test data
		Trex::<T>::send_trex_data(
				RawOrigin::Signed(caller.clone()).into(),
				ciphertext.clone(),
				release_time,
				key_pieces.clone()
			).unwrap();
	}: _(signed_caller, ciphertext.clone(),release_time, key_pieces.clone())

	send_expired_key{
		let caller: T::AccountId = whitelisted_caller();
		let signed_caller = RawOrigin::Signed(caller.clone());
		let key:Vec<u8> = EXPIRED_KEY.to_vec();
		let block_number = BLOCK_NUMBER.saturated_into();
		let ext_index = EXT_INDEX.saturated_into();
		Trex::<T>::send_expired_key(
				RawOrigin::Signed(caller.clone()).into(),
				key.clone(),
				block_number,
				ext_index
			).unwrap();
	}: _(signed_caller, key.clone(), block_number, ext_index)

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
