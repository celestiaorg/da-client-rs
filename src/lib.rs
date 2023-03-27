mod types;
mod header;
mod share;
mod node;
mod state;
mod p2p;

use jsonrpsee::{core::client::Subscription, ws_client::HeaderMap};
use crate::{
    state::StateClient,
    node::NodeClient,
    share::ShareClient,
    header::HeaderClient,
    p2p::P2PClient,
    types::{ExtendedHeader, DataAvailabilityHeader, ExtendedDataSquare}
};
use jsonrpsee::ws_client::WsClientBuilder;
use dotenv::dotenv;
use std::env;
use std::process::Command;
use std::str;

// TODO: network possibly still optional, but otherwise arabica does not work. apparently something else is default
pub fn generate_auth_token(node_type: &str, auth_level: &str) -> Result<String, std::io::Error> {
    println!("Generating auth token for {} with auth level {}", node_type, auth_level);
    let output = Command::new("celestia")
        .arg(node_type)
        .arg("auth")
        .arg(auth_level)
        .arg("--p2p.network")
        .arg("arabica-6")
        .output()?;

    if output.status.success() {
        let token = str::from_utf8(&output.stdout)
            .unwrap()
            .trim()
            .to_string();
        Ok(token)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to generate auth token",
        ))
    }
}

// charge mail mistake cannon brief subject crisp swear version library assist forum doll vehicle toast catch trim sponsor sauce hair cool dove jelly exercise
pub async fn run_client() {
    // Load the .env file
    dotenv().ok();
    
    // TODO: should light and read be the default?
    let node_type = env::var("NODE_TYPE").unwrap_or("light".to_string());
    let auth_level = env::var("AUTH_LEVEL").unwrap_or( "read".to_string());

    // Generate the auth token
    let jwt_token = generate_auth_token(&node_type.as_str(), &auth_level.as_str()).unwrap();

    // insert the auth token into the headers
    let mut headers = HeaderMap::new();
    headers.insert(http::header::AUTHORIZATION, http::header::HeaderValue::from_str(&format!("Bearer {}", jwt_token)).unwrap());

    let client = WsClientBuilder::default().set_headers(headers).build("ws://localhost:26658").await.unwrap();

    //println!("Node info: \n {:?}", NodeClient::node_info(&client).await.unwrap());
    println!("Header at height 1: \n {:?}", HeaderClient::get_by_height(&client, 1).await.unwrap());

    let mut sub: Subscription<ExtendedHeader> = HeaderClient::header_subscribe(&client).await.unwrap();
    while let Some(header) = sub.next().await {
        let dah = header.unwrap().dah;
        if dah.row_roots.len() > 2 {
            println!("Got header: \n {:?}", dah);
           let square = client.get_eds(dah).await.unwrap();
            println!("Found data square: \n {:?}", square);
        }
    } 
}