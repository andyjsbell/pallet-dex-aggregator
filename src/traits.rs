use sp_runtime::FixedU128;
use crate::pallet::Config;

pub trait Dex<T: Config> {

    fn get_quote(
        &self,
        from: &T::CurrencyId,
        to: &T::CurrencyId,
        amount: T::Balance) -> T::Balance;

    fn trading_pairs(&self) -> Vec<(T::CurrencyId, T::CurrencyId)>;
}

pub type DexList<T> = Vec<Box<dyn Dex<T>>>;
pub trait SetOfDex<T: Config> {
    fn get() -> DexList<T>;
}

pub type Path<Cur, Bal> = Vec<(Cur, Cur, Bal, Bal)>;

pub trait PathFinder<T: Config> {

    fn find_path(&mut self,
        from_token: T::CurrencyId,
        to_token: T::CurrencyId,
        amount: T::Balance,
        slippage: FixedU128,
    ) -> Path<T::CurrencyId, T::Balance>;
}