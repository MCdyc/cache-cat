use crate::network::network::{NetworkFactory, TcpNetwork};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Multi-Raft Router with per-node connection sharing.
#[derive(Clone, Default)]
pub struct Router {
    /// Map from node_id to node connection.
    /// All groups on the same node share this connection.
    pub nodes: Arc<RwLock<Vec<TcpNetwork>>>,
    pub addr: String,
}
impl Router {
    pub fn new(addr: String) -> Self {
        Self {
            nodes: Arc::new(RwLock::new(Vec::new())),
            addr,
        }
    }
    pub async fn register_node(&mut self, node_id: u64) {
        let net = NetworkFactory::new_tcp(node_id, self.addr.to_string()).await;
        self.nodes.write().await.push(net);
    }
}
