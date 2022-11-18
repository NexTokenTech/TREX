use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use codec::Encode;
use frame_benchmarking::{benchmarks, whitelisted_caller, Vec};
use frame_system::RawOrigin;
// local dependency
use benchmarking_primitives::generate_accounts;
use trex_primitives::{KeyPiece, ShieldedKey, TREXData};
use std::time::{SystemTime,UNIX_EPOCH};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let cipher = vec![1u8; 100];
		let key: ShieldedKey = vec![1u8; 32];
		// let key_piece = KeyPiece{holder: caller, shielded: key.clone()};
		let key_pieces = vec![];
		let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH) {
			Ok(n)=> n.as_secs(),
			Err(_) => 0
		};
		assert_ok!(TREXModule::send_trex_data(RuntimeOrigin::signed(1),cipher,timestamp,key_pieces));
	});
}
