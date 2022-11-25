/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

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
pub fn get_signer<AccountId: From<[u8; 32]>>(pubkey: &[u8; 32]) -> AccountId {
	AccountId::from(*pubkey)
}

pub mod consts {
	pub const TEST_CIPHER:&[u8] = &[137, 142, 28, 146, 3, 138, 141, 251, 127, 244, 112, 140, 114, 142, 129, 96, 108, 106, 85, 155, 105, 47, 29, 189, 250, 34, 6, 192, 83, 169, 20, 221, 97, 118];
	pub const SHIELDING_KEY:&[u8] = &[123, 123, 102, 224, 56, 141, 8, 150, 29, 116, 58, 140, 116, 179, 100, 127, 189, 181, 61, 188, 229, 120, 244, 54, 158, 178, 89, 99, 175, 72, 98, 202, 33, 147, 193, 90, 7, 247, 177, 45, 60, 5, 28, 29, 118, 131, 93, 221, 117, 40, 144, 152, 253, 127, 166, 171, 230, 9, 131, 189, 193, 63, 230, 37, 130, 184, 66, 28, 151, 8, 255, 193, 117, 21, 189, 79, 72, 22, 249, 140, 25, 227, 123, 91, 145, 156, 36, 93, 184, 74, 194, 100, 215, 36, 144, 125, 51, 35, 54, 11, 250, 40, 114, 67, 23, 158, 118, 248, 15, 245, 94, 216, 239, 120, 217, 44, 243, 254, 41, 222, 195, 153, 9, 16, 166, 111, 0, 152, 89, 142, 44, 190, 207, 165, 3, 252, 68, 154, 31, 179, 108, 164, 122, 48, 122, 19, 159, 218, 136, 46, 43, 97, 252, 217, 107, 227, 47, 4, 201, 210, 120, 6, 204, 69, 237, 30, 55, 131, 118, 28, 185, 197, 154, 44, 129, 18, 75, 186, 230, 243, 168, 178, 200, 84, 208, 161, 205, 153, 17, 191, 134, 40, 94, 62, 229, 12, 219, 220, 191, 168, 228, 216, 22, 139, 143, 181, 128, 100, 84, 242, 139, 156, 43, 123, 145, 29, 2, 64, 234, 51, 52, 151, 43, 48, 65, 173, 71, 218, 91, 67, 195, 135, 203, 165, 13, 112, 121, 188, 49, 65, 59, 16, 130, 65, 80, 177, 91, 235, 39, 232, 88, 251, 8, 10, 85, 82, 149, 114, 90, 194, 93, 246, 93, 81, 216, 181, 65, 12, 230, 138, 203, 159, 191, 126, 106, 179, 55, 108, 30, 42, 121, 201, 135, 252, 171, 86, 219, 148, 34, 168, 229, 20, 3, 223, 40, 139, 248, 62, 52, 126, 140, 131, 246, 130, 177, 14, 228, 19, 20, 157, 71, 206, 131, 190, 206, 8, 11, 191, 108, 221, 153, 216, 8, 138, 176, 114, 150, 178, 63, 87, 207, 239, 237, 236, 133, 243, 45, 249, 133, 18, 27, 133, 120, 144, 235, 126, 103, 161, 104, 175, 55, 121, 1, 106, 164, 83, 52, 250, 130, 98, 240, 175, 111, 202, 181, 54, 178, 10, 125, 251, 88, 87, 38, 52, 68, 12, 10, 56, 48, 220, 117, 182, 126, 119];
	pub const EXPIRED_KEY:&[u8] = &[123, 123, 102, 224, 56, 141, 8, 150, 29, 116, 58, 1];
	pub const BLOCK_NUMBER:u32 = 1u32;
	pub const EXT_INDEX: u32 = 1u32;

	// unix epoch. must be later than this
	pub const TEST_TIMESTAMP: u64 = 1669257689000;
	pub const TEST_SIGNER_PUB: &[u8; 32] =
		include_bytes!("./test-data/signing-pubkey-TEST.bin");
}
