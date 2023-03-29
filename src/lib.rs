mod types;
mod header;
mod share;
mod node;
mod state;
mod p2p;

use jsonrpsee::{ws_client::{HeaderMap, WsClient}};
use jsonrpsee::ws_client::WsClientBuilder;
use dotenv::dotenv;
use std::env;
use std::process::Command;
use std::str;
use http::header::{HeaderValue, AUTHORIZATION};


pub fn generate_auth_token(node_type: &str, auth_level: &str, network: &str) -> Result<String, std::io::Error> {
    println!("Generating auth token for {} with auth level {}", node_type, auth_level);
    let output = Command::new("celestia")
        .arg(node_type)
        .arg("auth")
        .arg(auth_level)
        .arg("--p2p.network")
        .arg(network)
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

fn build_auth_headers(auth_token: Option<String>) -> HeaderMap {
    // create a new header map to store the auth token
    let mut headers = HeaderMap::new();

    let token = match auth_token {
        // if an auth token is provided, use it
        Some(token) => token,
        None => {
            // if no auth token is provided, try to load it from the .env file
            dotenv().ok();
            env::var("AUTH_TOKEN").expect("Auth token not found in environment variables")
        }
    };

    // insert the auth token into the headers and return the headers
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token)).expect("Failed to create Authorization header value"));
    headers
}

pub struct CelestiaClient {
    pub client: WsClient,
}

impl CelestiaClient {
    // build a new public client without auth token
    pub async fn new_public_client(connection_string: &str) -> Self {
        let client = WsClientBuilder::default()
            .build(connection_string)
            .await
            .expect("Failed to build the WebSocket client");

        Self { client }
    }

    // build a new client auth token (if provided, otherwise try to load it from .env)
    pub async fn new_client(
        connection_string: &str,
        auth_token: Option<String>,
    ) -> Self {
        let headers = build_auth_headers(auth_token);
        let client = WsClientBuilder::default()
            .set_headers(headers)
            .build(connection_string)
            .await
            .expect("Failed to build the WebSocket client");

        Self { client }
    }
}



