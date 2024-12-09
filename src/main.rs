use near_tx_qx_builder::{NearTxSender, NearQxSender};
use serde_json::json;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    dotenv().ok();

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let account_id_sender = env::var("ACCOUNT_ID_SENDER").expect("ACCOUNT_ID_SENDER must be set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let account_id_receiver = env::var("ACCOUNT_ID_RECEIVER").expect("CONTRACT_ID must be set");

    // Transactions 
    let tx_sender = NearTxSender::builder(&rpc_url)
    .account_sender(&account_id_sender)
    .use_private_key(&private_key)
    .account_receiver(&account_id_receiver)
    .method_name("set_greeting")
    .args(json!({ "greeting": "Hello from FrameworkClienTHass" }))
    .build()?;


    let result = tx_sender.send_transaction().await?;
    println!("Transaction result: {:?}", result);

    // Queries
    let query_sender = NearQxSender::builder(&rpc_url)
    .account_sender(&account_id_sender)
    .account_receiver(&account_id_receiver)
    .method_name("get_greeting")
    .build()?;

    let query_result = query_sender.send_query().await?;
    println!("Query result: {}", query_result);

    Ok(())
}