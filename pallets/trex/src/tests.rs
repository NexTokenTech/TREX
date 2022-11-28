use crate::{mock::*,Error};
use frame_support::{assert_noop, assert_ok};
// local dependency
use trex_primitives::{KeyPiece, ShieldedKey};
use std::time::{SystemTime,UNIX_EPOCH};
use test_trex_utils::consts::*;
use sp_runtime::SaturatedConversion;

// give get_signer a concrete type
fn get_signer(pubkey: &[u8; 32]) -> AccountId {
	test_trex_utils::get_signer(pubkey)
}

#[test]
fn send_trex_data_works() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let caller = get_signer(TEST_SIGNER_PUB);
		let cipher = TEST_CIPHER.to_vec();
		let key: ShieldedKey = SHIELDING_KEY.to_vec();
		let key_piece = KeyPiece { holder: caller.clone(), shielded: key.clone() };
		let key_pieces = vec![key_piece];
		let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(n)=> n.as_secs(),
			Err(_) => 0
		};
		assert_ok!(TREXModule::send_trex_data(RuntimeOrigin::signed(caller),cipher,timestamp,key_pieces));
	});
}

#[test]
fn account_is_not_in_registry_works(){
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let caller = get_signer(TEST_SIGNER_PUB);
		let key: ShieldedKey = SHIELDING_KEY.to_vec();
		let block_number = BLOCK_NUMBER.saturated_into();
		let ext_index = EXT_INDEX.saturated_into();
		assert_noop!(
			TREXModule::send_expired_key(RuntimeOrigin::signed(caller),key,block_number,ext_index),
			Error::<Test>::EnclaveIsNotRegistered
		);
	});
}
