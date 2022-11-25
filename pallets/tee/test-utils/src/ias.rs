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

use core::default::Default;
use tee_primitives::Enclave;

pub trait TestEnclave<AccountId, Url,ShieldingKey> {
	fn test_enclave(pubkey: AccountId) -> Enclave<AccountId, Url, ShieldingKey>;
	fn with_mr_enclave(self, mr_enclave: [u8; 32]) -> Enclave<AccountId, Url,ShieldingKey>;
	fn with_timestamp(self, timestamp: u64) -> Enclave<AccountId, Url,ShieldingKey>;
	fn with_url(self, url: Url) -> Enclave<AccountId, Url,ShieldingKey>;
}

impl<AccountId, Url: Default, ShieldingKey: Default> TestEnclave<AccountId, Url,ShieldingKey> for Enclave<AccountId, Url,ShieldingKey> {
	fn test_enclave(pubkey: AccountId) -> Self {
		Enclave::new(
			pubkey,
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
			Default::default(),
		)
	}

	fn with_mr_enclave(mut self, mr_enclave: [u8; 32]) -> Self {
		self.mr_enclave = mr_enclave;
		self
	}

	fn with_timestamp(mut self, timestamp: u64) -> Self {
		self.timestamp = timestamp;
		self
	}

	fn with_url(mut self, url: Url) -> Self {
		self.url = url;
		self
	}
}

pub mod setups {
	use super::consts::*;

	#[derive(Copy, Clone)]
	pub struct IasSetup {
		pub cert: &'static [u8],
		pub signer_pub: &'static [u8; 32],
		pub mrenclave: [u8; 32],
		pub timestamp: u64,
	}

	pub const TEST4_SETUP: IasSetup = IasSetup {
		cert: TEST4_CERT,
		signer_pub: TEST4_SIGNER_PUB,
		mrenclave: TEST4_MRENCLAVE,
		timestamp: TEST4_TIMESTAMP,
	};

	pub const TEST5_SETUP: IasSetup = IasSetup {
		cert: TEST5_CERT,
		signer_pub: TEST5_SIGNER_PUB,
		mrenclave: TEST5_MRENCLAVE,
		timestamp: TEST5_TIMESTAMP,
	};

	pub const TEST6_SETUP: IasSetup = IasSetup {
		cert: TEST6_CERT,
		signer_pub: TEST6_SIGNER_PUB,
		mrenclave: TEST6_MRENCLAVE,
		timestamp: TEST6_TIMESTAMP,
	};

	pub const TEST7_SETUP: IasSetup = IasSetup {
		cert: TEST7_CERT,
		signer_pub: TEST7_SIGNER_PUB,
		mrenclave: TEST7_MRENCLAVE,
		timestamp: TEST7_TIMESTAMP,
	};
}

pub mod consts {
	use hex_literal::hex;

	pub const INCOGNITO_ACCOUNT: [u8; 32] = [
		44, 106, 196, 170, 141, 51, 4, 200, 143, 12, 167, 255, 252, 221, 15, 119, 228, 141, 94, 2,
		132, 145, 21, 17, 52, 41, 40, 220, 157, 130, 48, 176,
	];

	// reproduce with "integritee_service dump_ra"
	pub const TEST4_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST4.der");
	pub const TEST5_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST5.der");
	pub const TEST6_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST6.der");
	pub const TEST7_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST7.der");
	pub const TEST8_CERT: &[u8] = include_bytes!("./ias-data/ra_dump_cert_TEST8_PRODUCTION.der");

	// reproduce with integritee-service signing-key
	pub const TEST4_SIGNER_PUB: &[u8; 32] =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST4.bin");
	// equal to TEST4! because of MRSIGNER policy it was possible to change the MRENCLAVE but keep the secret
	pub const TEST5_SIGNER_PUB: &[u8; 32] =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST5.bin");
	pub const TEST6_SIGNER_PUB: &[u8; 32] =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST6.bin");
	pub const TEST7_SIGNER_PUB: &[u8; 32] =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST7.bin");
	pub const TEST8_SIGNER_PUB: &[u8; 32] =
		include_bytes!("./ias-data/enclave-signing-pubkey-TEST8-PRODUCTION.bin");

	// reproduce with "make mrenclave" in worker repo root
	// MRSIGNER is always 83d719e77deaca1470f6baf62a4d774303c899db69020f9c70ee1dfc08c7ce9e
	pub const TEST4_MRENCLAVE: [u8; 32] =
		hex!("7a3454ec8f42e265cb5be7dfd111e1d95ac6076ed82a0948b2e2a45cf17b62a0");
	pub const TEST5_MRENCLAVE: [u8; 32] =
		hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
	pub const TEST6_MRENCLAVE: [u8; 32] =
		hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
	pub const TEST7_MRENCLAVE: [u8; 32] =
		hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
	// production mode
	// MRSIGNER is 117f95f65f06afb5764b572156b8b525c6230db7d6b1c94e8ebdb7fba068f4e8
	pub const TEST8_MRENCLAVE: [u8; 32] =
		hex!("bcf66abfc6b3ef259e9ecfe4cf8df667a7f5a546525dee16822741b38f6e6050");

	// unix epoch. must be later than this
	pub const TEST4_TIMESTAMP: u64 = 1587899785000;
	pub const TEST5_TIMESTAMP: u64 = 1587900013000;
	pub const TEST6_TIMESTAMP: u64 = 1587900233000;
	pub const TEST7_TIMESTAMP: u64 = 1587900450000;
	pub const TEST8_TIMESTAMP: u64 = 1634156700000;

	pub const TWENTY_FOUR_HOURS: u64 = 60 * 60 * 24 * 1000;

	pub const URL: &[u8] =
		&[119, 115, 58, 47, 47, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 57, 57, 57, 49];

	pub const SHIELDING_KEY:&[u8] = &[123, 123, 102, 224, 56, 141, 8, 150, 29, 116, 58, 140, 116, 179, 100, 127, 189, 181, 61, 188, 229, 120, 244, 54, 158, 178, 89, 99, 175, 72, 98, 202, 33, 147, 193, 90, 7, 247, 177, 45, 60, 5, 28, 29, 118, 131, 93, 221, 117, 40, 144, 152, 253, 127, 166, 171, 230, 9, 131, 189, 193, 63, 230, 37, 130, 184, 66, 28, 151, 8, 255, 193, 117, 21, 189, 79, 72, 22, 249, 140, 25, 227, 123, 91, 145, 156, 36, 93, 184, 74, 194, 100, 215, 36, 144, 125, 51, 35, 54, 11, 250, 40, 114, 67, 23, 158, 118, 248, 15, 245, 94, 216, 239, 120, 217, 44, 243, 254, 41, 222, 195, 153, 9, 16, 166, 111, 0, 152, 89, 142, 44, 190, 207, 165, 3, 252, 68, 154, 31, 179, 108, 164, 122, 48, 122, 19, 159, 218, 136, 46, 43, 97, 252, 217, 107, 227, 47, 4, 201, 210, 120, 6, 204, 69, 237, 30, 55, 131, 118, 28, 185, 197, 154, 44, 129, 18, 75, 186, 230, 243, 168, 178, 200, 84, 208, 161, 205, 153, 17, 191, 134, 40, 94, 62, 229, 12, 219, 220, 191, 168, 228, 216, 22, 139, 143, 181, 128, 100, 84, 242, 139, 156, 43, 123, 145, 29, 2, 64, 234, 51, 52, 151, 43, 48, 65, 173, 71, 218, 91, 67, 195, 135, 203, 165, 13, 112, 121, 188, 49, 65, 59, 16, 130, 65, 80, 177, 91, 235, 39, 232, 88, 251, 8, 10, 85, 82, 149, 114, 90, 194, 93, 246, 93, 81, 216, 181, 65, 12, 230, 138, 203, 159, 191, 126, 106, 179, 55, 108, 30, 42, 121, 201, 135, 252, 171, 86, 219, 148, 34, 168, 229, 20, 3, 223, 40, 139, 248, 62, 52, 126, 140, 131, 246, 130, 177, 14, 228, 19, 20, 157, 71, 206, 131, 190, 206, 8, 11, 191, 108, 221, 153, 216, 8, 138, 176, 114, 150, 178, 63, 87, 207, 239, 237, 236, 133, 243, 45, 249, 133, 18, 27, 133, 120, 144, 235, 126, 103, 161, 104, 175, 55, 121, 1, 106, 164, 83, 52, 250, 130, 98, 240, 175, 111, 202, 181, 54, 178, 10, 125, 251, 88, 87, 38, 52, 68, 12, 10, 56, 48, 220, 117, 182, 126, 119];

}
