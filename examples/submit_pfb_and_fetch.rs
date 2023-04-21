extern crate da_client_rs;
use da_client_rs::{CelestiaClient, header::HeaderClient, state::StateClient};
use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

#[tokio::main]
async fn main() {
    // reads auth token from ENV
    let celestia_client = CelestiaClient::new_client("ws://localhost:26658", None).await;

    let namespace =  general_purpose::STANDARD.encode(vec![0,0,0,0,42,10,1,9]);
    let blob = general_purpose::STANDARD.encode(b"Hello, Celestia!");

    let pfb  = StateClient::submit_pay_for_blob(&celestia_client.client, namespace, blob, "2000".to_string(), 10000).await.unwrap();
    println!("Header at height 1: \n {:?}", HeaderClient::get_by_height(&celestia_client.client, 1).await.unwrap());
}