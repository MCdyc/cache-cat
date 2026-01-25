use crate::network::raft::TypeConfig;
use crate::server::client::client::RpcClient;
use crate::server::handler::model::InstallFullSnapshotReq;
use openraft::alias::VoteOf;
use openraft::error::{RPCError, ReplicationClosed, StreamingError};
use openraft::network::RPCOption;
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, SnapshotResponse, VoteRequest, VoteResponse,
};
use openraft::{
    BasicNode, OptionalSend, RaftNetworkFactory, RaftNetworkV2, RaftTypeConfig, Snapshot,
};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct NetworkFactory {}
impl RaftNetworkFactory<TypeConfig> for NetworkFactory {
    type Network = TcpNetwork;
    #[tracing::instrument(level = "debug", skip_all)]
    async fn new_client(&mut self, target: u64, node: &BasicNode) -> Self::Network {
        let client = RpcClient::connect(&*node.addr.clone()).await.unwrap();
        TcpNetwork {
            addr: node.addr.clone(),
            client,
            target,
        }
    }
}

pub struct TcpNetwork {
    addr: String,
    client: RpcClient,
    target: u64, //nodeid
}
impl TcpNetwork {
    async fn request<Req, Resp, Err>(
        &mut self,
        func_id: u32,
        req: Req,
    ) -> Result<Result<Resp, Err>, RPCError<TypeConfig>>
    where
        Req: Serialize + 'static,
        Resp: Serialize + DeserializeOwned,
        Err: std::error::Error + Serialize + DeserializeOwned,
    {
        let res: Result<Result<Resp, Err>, RPCError<TypeConfig>> =
            self.client.call(func_id, req).await.unwrap();
        res
    }
}

//openraft会自动调用这个方法，这里只需要实现网络层的rpc调用
impl RaftNetworkV2<TypeConfig> for TcpNetwork {
    async fn append_entries(
        &mut self,
        rpc: AppendEntriesRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<TypeConfig>, RPCError<TypeConfig>> {
        self.client.call(7, rpc).await.unwrap()
    }

    async fn vote(
        &mut self,
        rpc: VoteRequest<TypeConfig>,
        option: RPCOption,
    ) -> Result<VoteResponse<TypeConfig>, RPCError<TypeConfig>> {
        self.client.call(6, rpc).await.unwrap()
    }
    // 只是一个标识，并不真正进行快照
    async fn full_snapshot(
        &mut self,
        vote: VoteOf<TypeConfig>,
        mut snapshot: Snapshot<TypeConfig>,
        cancel: impl Future<Output = ReplicationClosed> + OptionalSend + 'static,
        option: RPCOption,
    ) -> Result<SnapshotResponse<TypeConfig>, StreamingError<TypeConfig>> {
        let data = snapshot.snapshot.into_inner();
        let req = InstallFullSnapshotReq {
            vote,
            snapshot_meta: snapshot.meta,
            snapshot: data,
        };
        self.client.call(8, req).await.unwrap()
    }
}
