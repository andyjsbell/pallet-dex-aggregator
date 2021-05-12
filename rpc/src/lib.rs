use pallet_dex_aggregator::traits::{TradingPair, Path, PathWithSlippage};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;

type TokenList = Vec<TradingPair<String>>;

#[rpc]
pub trait DexAggregatorApi<CurrencyId, Balance> {
    #[rpc(name = "dex_aggregator_getTokens")]
    fn get_tokens(&self) -> Result<TokenList>;
    #[rpc(name = "dex_aggregator_getQuote")]
    fn get_quote(&self, source_token: String, dest_token: String, amount: u64) -> Result<Path<CurrencyId, Balance>>;
}

struct DexAggregator;
impl<CurrencyId, Balance> DexAggregatorApi<CurrencyId, Balance> for DexAggregator {
    fn get_tokens(&self) -> Result<TokenList> {
        Ok(vec![])
    }
    fn get_quote(&self, source_token: String, dest_token: String, amount: u64) -> Result<Path<CurrencyId, Balance>> {
        Ok(vec![])
    }
}

#[test]
fn _get_tokens() {

}

#[test]
fn _get_quote() {

}