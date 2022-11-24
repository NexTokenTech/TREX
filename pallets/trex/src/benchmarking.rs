//! Benchmarking setup for pallet-trex

use super::*;

#[allow(unused)]
use crate::Pallet as Trex;
// substrate
use frame_benchmarking::{
	benchmarks, frame_support::sp_runtime::SaturatedConversion, whitelisted_caller, Vec,
};
use frame_system::RawOrigin;
use trex_primitives::{KeyPiece, ShieldedKey};

benchmarks! {
	where_clause {  where T::AccountId: From<[u8; 32]> }
	send_trex_data {
		let caller: T::AccountId = whitelisted_caller();
		// create cipher text
		let ciphertext = vec![137, 142, 28, 146, 3, 138, 141, 251, 127, 244, 112, 140, 114, 142, 129, 96, 108, 106, 85, 155, 105, 47, 29, 189, 250, 34, 6, 192, 83, 169, 20, 221, 97, 118];
		// construct key_pieces
		let key: ShieldedKey = vec![123, 123, 102, 224, 56, 141, 8, 150, 29, 116, 58, 140, 116, 179, 100, 127, 189, 181, 61, 188, 229, 120, 244, 54, 158, 178, 89, 99, 175, 72, 98, 202, 33, 147, 193, 90, 7, 247, 177, 45, 60, 5, 28, 29, 118, 131, 93, 221, 117, 40, 144, 152, 253, 127, 166, 171, 230, 9, 131, 189, 193, 63, 230, 37, 130, 184, 66, 28, 151, 8, 255, 193, 117, 21, 189, 79, 72, 22, 249, 140, 25, 227, 123, 91, 145, 156, 36, 93, 184, 74, 194, 100, 215, 36, 144, 125, 51, 35, 54, 11, 250, 40, 114, 67, 23, 158, 118, 248, 15, 245, 94, 216, 239, 120, 217, 44, 243, 254, 41, 222, 195, 153, 9, 16, 166, 111, 0, 152, 89, 142, 44, 190, 207, 165, 3, 252, 68, 154, 31, 179, 108, 164, 122, 48, 122, 19, 159, 218, 136, 46, 43, 97, 252, 217, 107, 227, 47, 4, 201, 210, 120, 6, 204, 69, 237, 30, 55, 131, 118, 28, 185, 197, 154, 44, 129, 18, 75, 186, 230, 243, 168, 178, 200, 84, 208, 161, 205, 153, 17, 191, 134, 40, 94, 62, 229, 12, 219, 220, 191, 168, 228, 216, 22, 139, 143, 181, 128, 100, 84, 242, 139, 156, 43, 123, 145, 29, 2, 64, 234, 51, 52, 151, 43, 48, 65, 173, 71, 218, 91, 67, 195, 135, 203, 165, 13, 112, 121, 188, 49, 65, 59, 16, 130, 65, 80, 177, 91, 235, 39, 232, 88, 251, 8, 10, 85, 82, 149, 114, 90, 194, 93, 246, 93, 81, 216, 181, 65, 12, 230, 138, 203, 159, 191, 126, 106, 179, 55, 108, 30, 42, 121, 201, 135, 252, 171, 86, 219, 148, 34, 168, 229, 20, 3, 223, 40, 139, 248, 62, 52, 126, 140, 131, 246, 130, 177, 14, 228, 19, 20, 157, 71, 206, 131, 190, 206, 8, 11, 191, 108, 221, 153, 216, 8, 138, 176, 114, 150, 178, 63, 87, 207, 239, 237, 236, 133, 243, 45, 249, 133, 18, 27, 133, 120, 144, 235, 126, 103, 161, 104, 175, 55, 121, 1, 106, 164, 83, 52, 250, 130, 98, 240, 175, 111, 202, 181, 54, 178, 10, 125, 251, 88, 87, 38, 52, 68, 12, 10, 56, 48, 220, 117, 182, 126, 119];
		let key_piece = KeyPiece { holder: caller.clone(), shielded: key.clone() };
		let key_pieces = vec![key_piece];

		let signed_caller = RawOrigin::Signed(caller.clone());
		let release_time:T::Moment = 1669257689000u64.saturated_into();
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
		let key:Vec<u8> = vec![123, 123, 102, 224, 56, 141, 8, 150, 29, 116, 58, 1];
		let block_number = 1u32.saturated_into();
		let ext_index = 1u32.saturated_into();
		Trex::<T>::send_expired_key(
				RawOrigin::Signed(caller.clone()).into(),
				key.clone(),
				block_number,
				ext_index
			).unwrap();
	}: _(signed_caller, key.clone(), block_number, ext_index)

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
