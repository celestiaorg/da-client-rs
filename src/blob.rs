use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::{Error};
use crate::types::{Blob, NamespaceId};

//TODO: Add missing methods
#[rpc(client)]
pub trait BlobMod {
    #[method(name = "blob.Submit")]
    async fn submit_blobs(&self, blobs: Vec<Blob>) -> Result<u64, Error>;

    #[method(name = "blob.GetAll")]
    async fn get_all_blobs(&self, height: u64, namespaces: Vec<NamespaceId>) -> Result<Vec<Blob>, Error>;
}
