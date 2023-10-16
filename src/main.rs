mod bot;
mod constants;
mod get_lnd;
mod utils;
use anyhow::Context;
use dotenv::dotenv;
use lnd_grpc_rust::lnrpc::GetInfoRequest;
use std::env;

use crate::bot::InitBot;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let log_level = env::var("LOG_LEVEL").unwrap_or("info".to_string());

    env::set_var("RUST_LOG", log_level);

    pretty_env_logger::init();

    let mut client = get_lnd::get_lnd()
        .await
        .context("Failed to get LND Client")?;

    // Make sure you are able to connect to lnd on start up
    client
        .lightning()
        .get_info(GetInfoRequest {})
        .await
        .expect("Failed to connect to Lnd");

    InitBot { client }.init().await;

    return Ok(());
}
