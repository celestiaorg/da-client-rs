use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::{Error};
use protobuf::Message;
use crate::proto::{byzantine};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProofType {
    BadEncoding
}

#[derive(Deserialize, Debug)]
pub struct ProofResponse {
    pub proof_type: String,
    pub data: Vec<u8>
}

impl ProofResponse {
    // TODO: Instead of this, create `Proof` trait and implement it for `BadEncoding`. Investigate
    // reusing nmt-rs for the proof validation.
    #[warn(dead_code)]
    pub fn to_befp(&self) -> Result<byzantine::BadEncoding, protobuf::Error> {
        byzantine::BadEncoding::parse_from_bytes(&self.data)
    }
}


#[rpc(client)]
pub trait Fraud {
    // fetches `FraudProof`s from the disk by its `ProofType`.
    #[method(name="fraud.Get")]
    async fn get_fraud(&self, proof_type: ProofType) -> Result<Vec<ProofResponse>, Error>;

    // subscribes to the `ProofType`'s pubsub topic.
    #[subscription(name = "fraud.Subscribe", unsubscribe = "unsub_fraud", item = ProofResponse)]
    fn fraud_subscribe(&self, proof_type: ProofType);
}