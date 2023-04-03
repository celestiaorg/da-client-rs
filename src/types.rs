use std::thread::sleep;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::core::{Error, client::Subscription};
use jsonrpsee::types::SubscriptionResult;
use jsonrpsee::ws_client::WsClientBuilder;
use serde::{Deserialize, Serialize};
use libp2p::{PeerId, Multiaddr};
use tendermint_rpc::endpoint::tx::DialectResponse as TxResponse;
use tendermint::account::Id;


#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendedDataSquare {
    pub codec: String,
    pub data_square: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendedHeader {
    pub header: Header,
    pub dah: DataAvailabilityHeader,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub height: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataAvailabilityHeader {
    pub row_roots: Vec<String>,
    pub column_roots: Vec<String>,
}
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct AddrInfo {
    pub id: PeerId,
    pub addrs: Vec<Multiaddr>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Stats {
    #[serde(rename = "TotalIn")]
    pub total_in: i64,
    #[serde(rename = "TotalOut")]
    pub total_out: i64,
    #[serde(rename = "RateIn")]
    pub rate_in: f64,
    #[serde(rename = "RateOut")]
    pub rate_out: f64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum Connectedness {
    NotConnected = 0,
    Connected = 1,
    CanConnect = 2,
    CannotConnect = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[repr(u8)]
pub enum Reachability {
    Unknown = 0,
    Public = 1,
    Private = 2,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    Public,
    Read,
    Write,
    Admin,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Info {
    #[serde(rename = "type")]
    pub node_type: Type,
    pub api_version: String,
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Copy)]
#[repr(u8)]
pub enum Type {
    Bridge = 1,
    Light = 2,
    Full = 3,
}
