use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::{Error};
use crate::types::{ExtendedHeader};

#[rpc(client)]
pub trait Header {
    #[method(name = "header.LocalHead")]
    async fn local_head(&self) -> Result<ExtendedHeader, Error>;

    #[method(name = "header.GetByHash")]
    async fn get_by_hash(&self, hash: String) -> Result<ExtendedHeader, Error>;

    #[method(name = "header.GetVerifiedRangeByHeight")]
    async fn get_verified_range_by_height(&self, from: ExtendedHeader) -> Result<Vec<ExtendedHeader>, Error>;

    #[method(name = "header.GetByHeight")]
    async fn get_by_height(&self, height: u64) -> Result<ExtendedHeader, Error>;

    // TODO: SyncState type
    // #[method(name = "header.SyncState")]
    // async fn sync_state(&self) -> Result<SyncState, Error>;

    #[method(name = "header.SyncWait")]
    async fn sync_wait(&self) -> Result<(), Error>;

    #[method(name = "header.NetworkHead")]
    async fn network_head(&self) -> Result<ExtendedHeader, Error>;

    #[subscription(name = "header.Subscribe", unsubscribe = "unsub_header", item = ExtendedHeader)]
    fn header_subscribe(&self);
}
