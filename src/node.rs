use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::{Error};
use crate::types::{Info, Permission};

#[rpc(client)]
pub trait Node {
    #[method(name = "node.Info")]
    async fn node_info(&self) -> Result<Info, Error>;

    #[method(name = "node.LogLevelSet")]
    async fn log_level_set(&self, name: String, level: String) -> Result<(), Error>;

    #[method(name = "node.AuthVerify")]
    async fn auth_verify(&self, token: String) -> Result<Vec<Permission>, Error>;

    #[method(name = "node.AuthNew")]
    async fn auth_new(&self, perms: Vec<Permission>) -> Result<Vec<u8>, Error>;
}
