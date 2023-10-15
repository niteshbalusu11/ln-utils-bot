use anyhow::Context;
use std::{env, fs};

pub async fn get_lnd() -> anyhow::Result<lnd_grpc_rust::LndClient> {
    let cert_path = env::var("CERT_PATH").context("failed to get cert")?;
    let macaroon_path = env::var("MACAROON_PATH").context("failed to get macaroon")?;
    let socket = env::var("SOCKET").context("failed to get socket")?;

    let cert = fs::read(cert_path).context("Failed to read cert file")?;
    let macaroon = fs::read(macaroon_path).context("Failed to read macaroon file")?;

    let client = lnd_grpc_rust::connect(hex::encode(cert), hex::encode(macaroon), socket)
        .await
        .expect("Failed to get LND Client");

    return Ok(client);
}
