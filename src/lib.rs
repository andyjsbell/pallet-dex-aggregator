#![cfg_attr(not(feature = "std"), no_std)]

mod mock;
mod tests;
mod traits;

pub use pallet::*;
use sp_runtime::FixedU128;
use sp_std::prelude::*;
use frame_support::{pallet_prelude::*};
use sp_runtime::{
    traits::{Zero},
};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_system::pallet_prelude::*;
    use crate::traits::*;
    use crate::mock::DEXs;
    use sp_runtime::traits::AtLeast32BitUnsigned;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type DEXs: SetOfDex<Self>;
        type CurrencyId: AtLeast32BitUnsigned + Clone;
        type Balance: AtLeast32BitUnsigned + Clone + Copy;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        SwapExecuted,
    }

    #[pallet::error]
    pub enum Error<T> {
        SwapFailed,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub c: T::BlockNumber,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig { c: Zero::zero() }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {

        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub(super) fn execute_swap(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }
    }

    impl<T: Config> PathFinder<T> for Pallet<T> {

        fn find_path(&mut self,
                     from_token: T::CurrencyId,
                     to_token: T::CurrencyId,
                     amount: T::Balance,
                     slippage: FixedU128) -> Path<T::CurrencyId, T::Balance>
        {
            // Dummy algorithm, TODO
            let dexs = DEXs::<T>::get();
            let mut paths = vec![];
            for dex in dexs {
                let target = dex.get_quote(&from_token, &to_token, amount);
                paths.push((from_token.clone(), to_token.clone(), amount.clone(), target));
            }
            paths
        }
    }
}