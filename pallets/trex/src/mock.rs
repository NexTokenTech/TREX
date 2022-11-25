use crate as pallet_trex;
pub use pallet_tee;
use frame_support::traits::{ConstU16, ConstU64};
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, IdentifyAccount, Verify},
};
pub use crate::weights::TREXWeight;

pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		TREXModule: pallet_trex,
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Tee: pallet_tee::{Pallet, Call, Config, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
		pub const MinimumPeriod: u64 = 6000 / 2;
}

pub type Moment = u64;

impl timestamp::Config for Test {
	type Moment = Moment;
	type OnTimestampSet = TREXModule;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const MomentsPerDay: u64 = 86_400_000; // [ms/d]
	pub const MaxSilenceTime: u64 =172_800_000; // 48h
}

pub type Balance = u64;

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxLocks = ();
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

impl pallet_tee::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = pallet_balances::Pallet<Test>;
	type MomentsPerDay = MomentsPerDay;
	type MaxSilenceTime = MaxSilenceTime;
}


impl pallet_trex::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type TREXWeight = pallet_trex::weights::SubstrateWeight<Test>;
	type EnclaveIndexStorage = Tee;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}