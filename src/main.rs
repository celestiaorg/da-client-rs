mod proto {
    pub mod byzantine;
    pub mod proof;
}
mod types;
mod fraud;
mod header;
mod share;
mod node;
mod state;
mod p2p;

use da_client_rs::{generate_auth_token, CelestiaClient};
use crate::{
    header::HeaderClient,
};

// example
#[tokio::main]
async fn main() {
    let token = generate_auth_token("light", "admin", "arabica-6").expect("Failed to generate auth token");
    println!("Token: {}", &token);

    let celestia_client = CelestiaClient::new_client("ws://localhost:26658", Some(token)).await;

    println!("Header at height 1: \n {:?}", HeaderClient::get_by_height(&celestia_client.client, 1).await.unwrap());
}