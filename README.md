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
    /// An approximation of costs in terms of commision and also any downstream costs
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

A proposed strategy would be:
- User sends an RPC request of their desired swap and the node runs the pathfinder algorithm returning the best path 
in less than 5 seconds.  
- The path is cached at the node for later retrieval. The path details plus a `quote_identifier` for this operation are returned to the caller.
- The user decides to execute the swap and calls the `swap` extrinsic with the `quote_identifier` and the source funds.
- The `quote_identifier` is pushed to pallet storage for execution in the next block.
- On the next block the OCW will process the `quote_identifier` by retrieving the path from the RPC search in the first step
- The OCW will call the `swap` extrinsic with the path details for execution
- On successful execution of the swap the callers account would be credited minus commission at X% rate
- On failure due to slippage or other constraints on the trade then the funds would be returned to the caller minus commission at X% rate
- On error any funds transferred by the caller would be sent back.

The commission is in place to protect against spam and to incentivise nodes running the network.
Alternatively the swap could be paid with a token native to the blockchain instead of a commission of the original token.
