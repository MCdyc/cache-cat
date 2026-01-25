use crate::handler::external_handler::HANDLER_TABLE;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use fory::Fory;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub async fn hand(
    tx: UnboundedSender<Bytes>,
    mut package: Bytes,
    fory: Arc<Fory>,
) -> Result<(), ()> {
    //回显消息,同样添加4 byte的长度头
    let request_id = u32::from_be_bytes(package[0..4].try_into().unwrap());
    let func_id = u32::from_be_bytes(package[4..8].try_into().unwrap());
    package.advance(8);
    //选择对应的方法并调用
    let handler = HANDLER_TABLE
        .iter()
        .find(|(id, _)| *id == func_id)
        .map(|(_, ctor)| ctor())
        .ok_or(())?;
    let response_data = handler.call(&fory, package);

    let mut response_length = response_data.len() as u32;
    response_length = response_length + 4;
    //BytesMut避免重复分配内存
    let mut response_header = BytesMut::with_capacity(8);
    response_header.put_u32(response_length);
    response_header.put_u32(request_id);
    response_header.put(response_data);
    let result = tx.send(response_header.freeze());
    if result.is_err() {
        return Err(());
    }
    Ok(())
}
