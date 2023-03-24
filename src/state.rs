use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::{Error};

// TODO: In general, many methods are missing, and don't use types from tendermint-rs like they should.
#[rpc(client)]
pub trait State {
    #[method(name = "state.AccountAddress")]
    async fn account_address(&self) -> Result<String, Error>;

    #[method(name = "state.Balance")]
    async fn balance(&self) -> Result<serde_json::Value, Error>;

    // TODO: Namespace type from sovereign labs?
    #[method(name = "state.SubmitPayForBlob")]
    async fn submit_pay_for_blob(&self, namespace: String, data: String, fee: String, gas_lim: u64) -> Result<serde_json::Value, Error>;
}