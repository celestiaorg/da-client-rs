use base64_serde::base64_serde_type;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use libp2p::{PeerId, Multiaddr};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtendedDataSquare {
    pub codec: String,
    pub data_square: Vec<String>,
}

base64_serde_type!(pub(crate) Base64Standard, base64::engine::general_purpose::STANDARD);

#[derive(Debug, Clone)]
pub struct NamespaceId(Vec<u8>);

impl NamespaceId {
    pub fn new_v0(hex: &str) -> Result<Self, hex::FromHexError> {
        let mut zero_padded_vec = vec![0; 21]; // 20 zero bytes
        zero_padded_vec.extend_from_slice(&hex::decode(hex)?);
        Ok(Self(zero_padded_vec))
    }
}

impl Serialize for NamespaceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        Base64Standard::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for NamespaceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(NamespaceId(Base64Standard::deserialize(deserializer)?))
    }
}


#[derive(Serialize, Deserialize)]
pub struct Blob {
    pub namespace: NamespaceId,
    #[serde(with = "Base64Standard")]
    pub data: Vec<u8>,
    pub share_version: u32,
    pub namespace_version: u32,
    #[serde(with = "Base64Standard")]
    pub commitment: Vec<u8>
}

impl Blob {
    pub fn new<T: Serialize>(namespace: NamespaceId, data: T) -> Result<Self, serde_json::error::Error> {
        let data = serde_json::to_string(&data)?;
        Ok(Self {
            namespace,
            data: data.into_bytes(),
            share_version: 0,
            namespace_version: 0,
            // TODO: add non-empty commitment once we have a way to compute it in rust
            commitment: vec![],
        })
    }
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
