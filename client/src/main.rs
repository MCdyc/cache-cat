use client::client::RpcClient;
use server::share::model::{GetReq, GetRes, SetReq, SetRes};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RpcClient::connect("127.0.0.1:8080").await?;

    // let res2: SetRes = client
    //     .call(
    //         2,
    //         SetReq {
    //             key: "key".to_string(),
    //             value: Vec::from("value".to_string()),
    //             ex_time: 1000000,
    //         },
    //     )
    //     .await?;

    let res2: SetRes = client
        .call(
            2,
            SetReq {
                key: "key".to_string(),
                value: Vec::from("val11111ue".to_string()),
                ex_time: 10000,
            },
        )
        .await?;

    let res3: GetRes = client
        .call(
            3,
            GetReq {
                key: "key".to_string(),
            },
        )
        .await?;
    match res3 {
        GetRes {
            value: Some(arc_vec),
        } => {
            // 使用 from_utf8_lossy 处理无效的 UTF-8 序列
            let s = String::from_utf8_lossy(&arc_vec);
            println!("{}", s);
        }
        GetRes { value: None } => {
            println!("No value");
        }
    }
    Ok(())

}
