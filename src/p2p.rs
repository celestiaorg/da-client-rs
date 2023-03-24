use jsonrpsee::proc_macros::rpc;
use jsonrpsee::core::{Error};
use libp2p::PeerId;
use crate::types::{AddrInfo, Connectedness, Reachability, Stats};

#[rpc(client)]
pub trait P2P {
    #[method(name = "p2p.Info")]
    async fn p2p_info(&self) -> Result<AddrInfo, Error>;

    #[method(name = "p2p.Peers")]
    async fn peers(&self) -> Result<Vec<PeerId>, Error>;

    #[method(name = "p2p.PeerInfo")]
    async fn peer_info(&self, id: PeerId) -> Result<AddrInfo, Error>;

    #[method(name = "p2p.Connect")]
    async fn connect(&self, addr_info: AddrInfo) -> Result<(), Error>;

    #[method(name = "p2p.ClosePeer")]
    async fn close_peer(&self, id: PeerId) -> Result<(), Error>;

    #[method(name = "p2p.Connectedness")]
    async fn connectedness(&self, id: PeerId) -> Result<Connectedness, Error>;

    #[method(name = "p2p.NATStatus")]
    async fn nat_status(&self) -> Result<Reachability, Error>;

    #[method(name = "p2p.BlockPeer")]
    async fn block_peer(&self, peer: PeerId) -> Result<(), Error>;

    #[method(name = "p2p.UnblockPeer")]
    async fn unblock_peer(&self, peer: PeerId) -> Result<(), Error>;

    #[method(name = "p2p.ListBlockedPeers")]
    async fn list_blocked_peers(&self) -> Result<Vec<PeerId>, Error>;

    #[method(name = "p2p.Protect")]
    async fn protect(&self, id: PeerId, tag: String) -> Result<(), Error>;

    #[method(name = "p2p.Unprotect")]
    async fn unprotect(&self, id: PeerId, tag: String) -> Result<bool, Error>;

    #[method(name = "p2p.IsProtected")]
    async fn is_protected(&self, id: PeerId, tag: String) -> Result<bool, Error>;

    #[method(name = "p2p.BandwidthStats")]
    async fn bandwidth_stats(&self) -> Result<Stats, Error>;

    #[method(name = "p2p.BandwidthForPeer")]
    async fn bandwidth_for_peer(&self, id: PeerId) -> Result<Stats, Error>;

    #[method(name = "p2p.BandwidthForProtocol")]
    async fn bandwidth_for_protocol(&self, proto: String) -> Result<Stats, Error>;

    // TODO: Implement ResourceManagerStat type
    // #[method(name = "p2p.ResourceState")]
    // async fn resource_state(&self) -> Result<ResourceManagerStat, Error>;

    #[method(name = "p2p.PubSubPeers")]
    async fn pubsub_peers(&self, topic: String) -> Result<Vec<PeerId>, Error>;
}