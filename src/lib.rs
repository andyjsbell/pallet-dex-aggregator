#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::{pallet_prelude::*};
use sp_runtime::{
    traits::{Zero},
};

mod mock;
mod tests;

pub use module::*;

#[frame_support::pallet]
pub mod module {
    use super::*;
    use frame_system::pallet_prelude::OriginFor;

    #[pallet::config]
    pub trait Config: frame_system::Config {
    }

    /// Error for non-fungible-token module.
    #[pallet::error]
    pub enum Error<T> {
        None,
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

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub(super) fn something(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {

}