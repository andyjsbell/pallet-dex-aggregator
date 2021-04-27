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
type Fix = FixedU128<U4>;
struct Params {
    slippage: Fix,
    other: bool, // TBD
}

trait Config  {
    type CurrencyId;
    type Balance;
    type Costs;
}

type Dex<T> = Box<dyn DEX<T>>;
trait PathFinder<T: Config> {
    fn find_path(
        from_token: T::CurrencyId, 
        to_token: T::CurrencyId, 
        amount: T::Balance, 
        dex: Vec<Dex<T>>,       
        params: Params,
    );
}

```

and for a DEX:

```
type TradingPair<C> = (C, C);
type TradingPairs<C> = Vec<TradingPair<C>>;
trait DEX<T: Config> {
    /// Get a list of support trading pairs from the DEX
    fn trading_pairs(&self) -> Option<TradingPairs<T::CurrrencyId>>;
    /// Get a quote from the DEX, returning target amount and cost of operation
    fn get_quote(
        from_token: T::CurrencyId, 
        to_token: T::CurrencyId, 
        amount: T::Balance
    ) -> (T::Balance, T::Costs);
}
```

Without encrypting the mempool and extrinsic calls front-running is a danger so we have to be prevent exposing the 
intended trade until we wish to execute the trade.  

A quote would be provided for a swap via an RPC call on the node in which the swap path would be stored in memory in
the same node and a "quote reference" to this resulting path would be returned.  The quote could then be executed by 
submitting a signed transaction with the "quote reference" and funds.  This would schedule the quote to be ran in the OCW.  
The OCW then on the next block will call up the "quote reference" via RPC and the resulting path returned from RPC would
execute the swap by calling the `swap` extrinsic.

If the swap is executed correctly then the event `SwapExecuted(from_token, to_token, amount, target_amount)` would be 
emitted and if it fails due to parameters set on the swap then the event `SwapFailed(from_token, to_token, amount)` would
be emitted.

The amount to be swapped would be transferred to the pallet and held by the pallet until either the swap is executed or
if the swap fails. For a successful swap the target token would be transferred to the swap caller.  For a failure the 
original swap token would be returned to the caller.  In order to protect against spam and to incentivise nodes a commission
would be charged at 0.3% of the original token for a successful swap or 0.03% for a swap that failed.

Alternatively the swap could be paid with a token native to the blockchain instead of a commission of the original token.

