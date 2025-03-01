use super::*;

use crate as pallet_community_loan_pool;
use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use sp_core::ConstU32;
use sp_runtime::{
	traits::{AccountIdLookup, BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature,
};

use pallet_transaction_payment::CurrencyAdapter;

use frame_support::traits::ConstU8;

use frame_support::weights::IdentityFee;

use pallet_transaction_payment::{ConstFeeMultiplier, Multiplier};

use sp_runtime::traits::One;

use sp_runtime::BuildStorage;

use pallet_nfts::PalletFeatures;

pub type Block = frame_system::mocking::MockBlock<Test>;

pub type BlockNumber = u64;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Signature = MultiSignature;
pub type AccountPublic = <Signature as Verify>::Signer;

/* let id = [0: u32].into();

pub const ALICE: AccountId = id;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DAVE: AccountId = 4;  */

pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Uniques: pallet_nfts::{Pallet, Call, Storage, Event<T>},
		CommunityLoanPool: pallet_community_loan_pool,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Randomness: pallet_insecure_randomness_collective_flip::{Pallet, Storage},
		TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
}

impl frame_system::Config for Test {
	type RuntimeCall = RuntimeCall;
	type Nonce = u32;
	type Block = Block;
	type Hash = sp_core::H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u32>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type SystemWeightInfo = ();
	type BlockWeights = ();
	type BlockLength = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = u32;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU32<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	// Holds are used with COLLATOR_LOCK_ID and DELEGATOR_LOCK_ID
	type MaxHolds = ConstU32<2>;
	type MaxFreezes = ConstU32<0>;
}

parameter_types! {
	pub Features: PalletFeatures = PalletFeatures::all_enabled();
	pub const ApprovalsLimit: u32 = 20;
	pub const ItemAttributesApprovalsLimit: u32 = 20;
	pub const MaxTips: u32 = 10;
	pub const MaxDeadlineDuration: BlockNumber = 12 * 30 * DAYS;
	pub const MaxAttributesPerCall: u32 = 10;
}

impl pallet_nfts::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<Self::AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Locker = ();
	type CollectionDeposit = ConstU32<2>;
	type ItemDeposit = ConstU32<1>;
	type MetadataDepositBase = ConstU32<1>;
	type AttributeDepositBase = ConstU32<1>;
	type DepositPerByte = ConstU32<1>;
	type StringLimit = ConstU32<50>;
	type KeyLimit = ConstU32<50>;
	type ValueLimit = ConstU32<50>;
	type WeightInfo = ();
	type ApprovalsLimit = ApprovalsLimit;
	type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = Features;
	type OffchainSignature = Signature;
	type OffchainPublic = AccountPublic;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
}

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = IdentityFee<u32>;
	type LengthToFee = IdentityFee<u32>;
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

impl pallet_insecure_randomness_collective_flip::Config for Test {}

parameter_types! {
	pub const MinimumPeriod: u64 = 1;
}
impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const CommunityLoanPalletIdPalletId: PalletId = PalletId(*b"py/cmmty");
	pub const MaxLoans: u32 = 10000;
	pub const VotingTime: BlockNumber = 10;
	pub const MaximumCommitteeMembers: u32 = 10;
	pub const MaxMilestones: u32 = 10;
}

impl pallet_community_loan_pool::Config for Test {
	type PalletId = CommunityLoanPalletIdPalletId;
	type Currency = Balances;
	type ApproveOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type RejectOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type CommitteeOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type DeleteOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ConstU32<10000>;
	type ProposalBondMaximum = ();
	type OnSlash = ();
	type MaxOngoingLoans = MaxLoans;
	type TimeProvider = Timestamp;
	type WeightInfo = weights::SubstrateWeight<Test>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = NftHelper;
	type VotingTime = VotingTime;
	type MaxCommitteeMembers = MaximumCommitteeMembers;
	type MaxMilestonesPerProject = MaxMilestones;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut test = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			([0; 32].into(), 20_000_000),
			([1; 32].into(), 15_000),
			([2; 32].into(), 150_000),
			([3; 32].into(), 5_000),
			((CommunityLoanPool::account_id()), 20_000_000),
		],
	}
	.assimilate_storage(&mut test)
	.unwrap();

	test.into()
}
