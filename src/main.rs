mod types;
mod header;
mod share;
mod node;
mod state;
mod p2p;

use jsonrpsee::core::client::Subscription;
use crate::{
    state::StateClient,
    node::NodeClient,
    share::ShareClient,
    header::HeaderClient,
    p2p::P2PClient,
    types::{ExtendedHeader, DataAvailabilityHeader, ExtendedDataSquare}
};
use jsonrpsee::ws_client::WsClientBuilder;

#[tokio::main]
async fn main() {
    let client = WsClientBuilder::default().build("ws://localhost:26658").await.unwrap();
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