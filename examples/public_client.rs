extern crate da_client_rs;
use da_client_rs::{CelestiaClient, header::HeaderClient};

#[tokio::main]
async fn main() {
    let celestia_client = CelestiaClient::new_public_client("ws://localhost:26658").await;
    println!("Network Head: \n {:?}", HeaderClient::network_head(&celestia_client.client).await.unwrap());
}