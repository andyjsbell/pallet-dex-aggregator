
use super::*;
use crate as pallet_dex_aggregator;
use sp_core::H256;
use sp_runtime::{
    Perbill,
    traits::{
        BlakeTwo256,
        IdentityLookup,
    },
    testing::{
        Header,
    },
};
use frame_support::{parameter_types, construct_runtime,};
use crate::traits::*;
use sp_std::hash::Hash;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		DexAggregator: pallet_dex_aggregator::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

pub struct DEXs<T: Config> {
    _marker: PhantomData<T>,
}

// Our tokens
type CurrencyId = u32;
const TOKEN_1: CurrencyId = 1;
const TOKEN_2: CurrencyId = 2;
const TOKEN_3: CurrencyId = 3;
const TOKEN_4: CurrencyId = 4;
const TOKEN_5: CurrencyId = 5;

// Create two DEX (A and B).
// We can't trade TOKEN_1 for TOKEN_5 without going via TOKEN_2 which can be bought
// on DEX_A and then exchanged for TOKEN_5 on DEX_B
struct DEX_A;
struct DEX_B;

impl<T:Config> Dex<T> for DEX_A {
    fn get_quote(
        &self,
        from: &T::CurrencyId,
        to: &T::CurrencyId,
        amount: T::Balance) -> T::Balance {
        amount
    }

    fn trading_pairs(&self) -> Vec<(T::CurrencyId, T::CurrencyId)> {
        vec![
            (TOKEN_1.into(), TOKEN_2.into()),
            (TOKEN_1.into(), TOKEN_3.into()),
            (TOKEN_1.into(), TOKEN_4.into()),
        ]
    }
}

impl<T:Config> Dex<T> for DEX_B {
    fn get_quote(
        &self,
        from: &T::CurrencyId,
        to: &T::CurrencyId,
        amount: T::Balance) -> T::Balance {
        amount
    }

    fn trading_pairs(&self) -> Vec<(T::CurrencyId, T::CurrencyId)> {
        vec![(TOKEN_2.into(), TOKEN_5.into())]
    }
}

// Mock a set of DEXs for us to run the `PathFinder` with
impl<T: Config> SetOfDex<T> for DEXs<T> {
    fn get() -> DexList<T> {
        vec![Box::new(DEX_A{}), Box::new(DEX_B{})]
    }
}

impl Config for Test {
    type Event = Event;
    type DEXs = DEXs<Self>;
    type CurrencyId = CurrencyId;
    type Balance = u64;
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    frame_system::GenesisConfig::default().assimilate_storage::<Test>(&mut t).unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}

