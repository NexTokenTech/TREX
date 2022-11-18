#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

// TODO: Adjust the weight equations.
/// Weight functions needed for pallet_balances.
pub trait TREXWeight {
    fn send_trex_data() -> u64;
    fn do_something() -> u64;
}

/// Weights for pallet_balances using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> TREXWeight for SubstrateWeight<T> {
    // Storage: System Account (r:1 w:1)
    fn send_trex_data() -> u64 {
        70_952_000 + T::DbWeight::get().reads_writes(1,1).ref_time()
    }
    fn do_something() -> u64 {
        70_952_000 + T::DbWeight::get().reads_writes(1,1).ref_time()
    }
}

// For backwards compatibility and tests
impl TREXWeight for () {
    // Storage: System Account (r:1 w:1)
    fn send_trex_data() -> u64 {
        70_952_000 + RocksDbWeight::get().reads_writes(1, 1).ref_time()
    }
    fn do_something() -> u64 {
        70_952_000 + RocksDbWeight::get().reads_writes(1, 1).ref_time()
    }
}