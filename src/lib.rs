#![cfg_attr(not(feature = "std"), no_std)]

mod mock;
mod tests;
pub mod traits;

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
        type CurrencyId: Parameter + AtLeast32BitUnsigned + Clone;
        type Balance: Parameter + AtLeast32BitUnsigned + Clone + Copy;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        SwapExecuted,
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidSwap,
        SwapFailed,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig {
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            GenesisConfig { }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub(super) fn execute_swap(origin: OriginFor<T>, path: Path<T::CurrencyId, T::Balance>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            // Run through path and make swaps
            // Confirm we have the source amount token
            // Iterate along path to make the swaps
            // This would need to be atomic and would rollback on any failure during the swap
            // If we hit slippage fail and rollback
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
            // Dummy algorithm, just gets quotes and returns them
            // Would need to then build a path with these TODO
            let dexs = DEXs::<T>::get();
            let mut paths = vec![];
            for dex in dexs {
                let target = dex.get_quote(&from_token, &to_token, amount);
                paths.push((from_token.clone(), to_token.clone(), amount.clone(), target));
            }
            (paths, slippage)
        }
    }
}