# pallet-dex-aggregator

**WIP**

This is a proof-of-concept implementation of a DEX aggregator pallet for Substrate and FRAME.  https://github.com/paritytech/substrate

## Background

1inch (https://1inch.io/) provides a service to aggregate DEXs to find the best path for a swap a user is looking for to complete. 

The 1inch protocol currently supports Ethereum and the Binance Smart Chain.  It's implementation can be found on Github: https://github.com/1inch/1inchProtocol.

It searches for the best path by integrating with several DEX protocols such UniSwap all onchain and hence decentralised.

More information can be found here https://github.com/1inch/1inchProtocol/blob/master/README.md

## Design

In order to find the best path a path finder algorithm would be used which would run in an offchain worker(OCW) as the algorithm would be non-deterministic.

We would have one extrinsic for the user:

`fn swap(origin, from_token, to_token, amount, options)`

This would schedule the trade for the next block and store this in an ordered list of `Swap` structure.

```
struct Swap<AccountId, CurrencyId, Options> {
    caller: AccountId,
    to_token: CurrencyId,
    from_token: CurrencyId,
    options: Options
}
```

On the next block the OCW would be called and this would run through the swaps that had been submitted and execute on each.  For each swap it would pass the trade through the path finder algorithm and if slippage and options are acceptable for this path it would create a transaction calling the pallet to execute the path.

The path finder trait would be defined as follows:

```
struct Params {
    slippage: f32,
    other: bool, // TBD
}

trait Config  {
    type CurrencyId;
    type Balance;
}

type Dex<T> = Vec<Box<dyn DEX<T>>>;
trait PathFinder<T: Config> {
    fn find_path(
        from_token: T::CurrencyId, 
        to_token: T::CurrencyId, 
        amount: T::Balance, 
        dex: Dex<T>,
        params: Params,
    );
}

type Pool<C,A> = Vec<(C, A, C, A)>;
trait DEX<T: Config> {
    fn pools(&self) -> Pool<T::CurrencyId, T::Balance>;
}

```

and for a DEX:

```
type Pool<C,A> = Option<Vec<(C, A, C, A)>>;
trait DEX<T: Config> {
    fn pools(&self) -> Pool<T::CurrencyId, T::Amount>;
}
```

On accepting a swap request the pallet would send the `SwapAccepted(from_token, to_token, amount)` event.  If the swap,
when it is processed, is executed then the event `SwapExecuted(from_token, to_token, amount, target_amount)` and if it fails due to
parameters set on the swap then the event `SwapFailed(from_token, to_token, amount)`
