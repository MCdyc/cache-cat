use crate::network::model::{Request, Response};
use crate::network::network::NetworkFactory;
use crate::server::handler::rpc;
use openraft::Config;
use std::io::Cursor;
use std::sync::Arc;

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig:
        D = Request,
        R = Response,
        Entry = openraft::Entry<TypeConfig>,
        SnapshotData = Cursor<Vec<u8>>,
);
pub type Raft = openraft::Raft<TypeConfig>;
//实现是纯内存的暂时
pub type LogStore = crate::store::log::LogStore;
pub type StateMachineStore = crate::store::state_machine::StateMachineStore<TypeConfig>;
pub async fn start_raft_app(node_id: u64, addr: String) -> std::io::Result<()> {
    let config = Arc::new(Config {
        heartbeat_interval: 250,
        election_timeout_min: 299,
        ..Default::default()
    });
    let log_store = LogStore::default();
    let state_machine_store = StateMachineStore::default();
    //客户端网络
    let network = NetworkFactory {};
    // Create a local raft instance.
    let raft = openraft::Raft::new(
        node_id,
        config.clone(),
        network,
        log_store,
        state_machine_store.clone(),
    )
    .await
    .unwrap();
    let app = CacheCatApp {
        id: node_id,
        addr: addr.clone(),
        raft,
        config,
    };
    //服务端网络
    rpc::start_server(Arc::new(app)).await
}
pub struct CacheCatApp {
    pub id: u64,
    pub addr: String,
    pub raft: Raft,
    pub config: Arc<Config>,
}
