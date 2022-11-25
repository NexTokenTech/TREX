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

use frame_support::dispatch::DispatchErrorWithPostInfo;
use frame_support::ensure;
use frame_support::traits::OnTimestampSet;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;
pub mod weights;
pub use weights::TREXWeight;
use trex_primitives::{TREXData,TREXExpiredKey, KeyPiece, MAX_TREX_DATA};
use sp_std::vec;
use pallet_tee::TeeStorageInterface;

// TODO: add supports for try-runtime test.

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::dispatch::DispatchClass;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + timestamp::Config{
		/// Because this pallet emits events, it depends on the runtime definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		type TREXWeight: TREXWeight;

		type EnclaveIndexStorage: TeeStorageInterface<Value = bool,AccountId = <Self as frame_system::Config>::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// TREX Data Send Event
		TREXDataSent(T::AccountId, Vec<u8>),
		TREXExpiredKeySent(T::AccountId, Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		// TODO: add overflow check.
		/// Errors should have helpful documentation associated with them.
		TREXInfoSentOverflow,
		EnclaveIsNotRegistered
	}

	#[pallet::genesis_config]
	#[cfg_attr(feature = "std", derive(Default))]
	pub struct GenesisConfig {}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		/// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		#[pallet::weight(T::TREXWeight::send_trex_data())]
		pub fn send_trex_data(
			origin: OriginFor<T>,
			cipher: Vec<u8>,
			release_time: T::Moment,
			key_pieces: Vec<KeyPiece<T::AccountId>>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// construct InfoData Struct for TREXStorage
			let owner = who.clone();
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let trex_data = TREXData::<T::AccountId, T::Moment, T::BlockNumber>{
				cipher,
				from: owner,
				release_time,
				current_block: current_block_number,
				key_pieces
			};

			//encode InfoData instance to vec<u8>
			let trex_byte_data = trex_data.encode();
			let trex_data_size = trex_byte_data.len();
			ensure!(trex_data_size < *MAX_TREX_DATA.max.get(DispatchClass::Normal) as usize, <Error<T>>::TREXInfoSentOverflow);

			// Emit an event.
			Self::deposit_event(Event::TREXDataSent(who, trex_byte_data));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(T::TREXWeight::send_trex_data())]
		pub fn send_expired_key(
			origin: OriginFor<T>,
			expired_key: Vec<u8>,
			block_number: T::BlockNumber,
			ext_index: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			#[cfg(not(feature = "runtime-benchmarks"))]
			Self::is_registered_enclave(&who)?;
			// construct InfoData Struct for TREXStorage
			let owner = who.clone();

			let trex_expired_key = TREXExpiredKey::<T::AccountId, T::BlockNumber>{
				expired_key,
				from: owner,
				block_number,
				ext_index
			};

			//encode InfoData instance to vec<u8>
			let trex_byte_expired_key = trex_expired_key.encode();
			// Emit an event.
			Self::deposit_event(Event::TREXExpiredKeySent(who, trex_byte_expired_key));
			// Return a successful DispatchResultWithPostInfo

			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Check if the sender is a registered enclave
	pub fn is_registered_enclave(
		account: &T::AccountId,
	) -> Result<bool, DispatchErrorWithPostInfo> {
		let v = T::EnclaveIndexStorage::contains_account(account);
		ensure!(v == true, <Error<T>>::EnclaveIsNotRegistered);
		Ok(true)
	}
}

impl<T: Config> OnTimestampSet<T::Moment> for Pallet<T> {
	fn on_timestamp_set(_moment: T::Moment) {

	}
}
