#[cfg(test)]
mod tests {
    use crate::network::model::Request;
    use crate::network::raft_rocksdb::TypeConfig;
    use crate::server::client::client::RpcClient;
    use crate::server::handler::model::SetReq;
    use openraft::raft::ClientWriteResponse;

    #[tokio::test]
    async fn test_add() {
        let mut client = RpcClient::connect("127.0.0.1:3003").await.unwrap();
        let res: ClientWriteResponse<TypeConfig> = client
            .call(
                2,
                Request::Set(SetReq {
                    key: "test".to_string(),
                    value: Vec::from("test_value".to_string()),
                    ex_time: 0,
                }),
            )
            .await
            .expect("call failed");
        let res: Option<String> = client
            .call(3, "test".to_string())
            .await
            .expect("call failed");
        println!("res: {:?}", res.expect("res is none"))
    }
}
