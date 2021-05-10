#[rpc]
pub trait DexAggregatorApi {
    #[rpc(name = "dex_getTokens")]
    fn get_tokens(&self) -> Result<ResponseType>;
}