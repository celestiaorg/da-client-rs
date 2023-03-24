use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::{Error};
use crate::types::{DataAvailabilityHeader, ExtendedDataSquare};

#[rpc(client)]
pub trait Share {
    #[method(name = "share.SharesAvailable")]
    async fn shares_available(&self, root: DataAvailabilityHeader) -> Result<(), Error>;

    #[method(name = "share.ProbabilityOfAvailability")]
    async fn probability_of_availability(&self) -> Result<f64, Error>;

    #[method(name = "share.GetShare")]
    async fn get_share(&self, dah: DataAvailabilityHeader, row: isize, col: isize) -> Result<String, Error>;

    #[method(name = "share.GetEDS")]
    async fn get_eds(&self, dah: DataAvailabilityHeader) -> Result<ExtendedDataSquare, Error>;

    // TODO: NamespacedShares type
    #[method(name = "share.GetSharesByNamespace")]
    async fn get_shares_by_namespace(&self, dah: DataAvailabilityHeader, namespace: String) -> Result<serde_json::Value, Error>;
}
