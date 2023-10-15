use anyhow::ensure;
use lightning_probing::{probe_destination, ProbeDestination};
use lnd_grpc_rust::lnrpc::{self, ConnectPeerRequest, DisconnectPeerRequest, LightningAddress};

use anyhow::Result;

pub async fn connect_peer(
    client: &mut lnd_grpc_rust::LndClient,
    addr: &str,
) -> Result<lnrpc::ConnectPeerResponse> {
    let (pubkey, host) = parse_address(addr)?;

    disconnect_peer(client, &pubkey).await;

    log::info!("Connecting to peer {}", &pubkey);

    let address = LightningAddress {
        host: host.clone(),
        pubkey: pubkey.clone(),
    };

    let res = client
        .lightning()
        .connect_peer(ConnectPeerRequest {
            addr: Some(address),
            ..Default::default()
        })
        .await?
        .into_inner();

    disconnect_peer(client, &pubkey).await;

    Ok(res)
}

fn parse_address(addr: &str) -> Result<(String, String)> {
    ensure!(!addr.is_empty(), "Address cannot be empty");

    let parts: Vec<&str> = addr.split('@').collect();

    ensure!(parts.len() >= 2, "Missing pubkey/uri");
    ensure!(!parts[0].is_empty(), "Missing pubkey");
    ensure!(!parts[1].is_empty(), "Missing socket");

    Ok((parts[0].to_string(), parts[1].to_string()))
}

pub async fn disconnect_peer(client: &mut lnd_grpc_rust::LndClient, pubkey: &str) {
    let _ = client
        .lightning()
        .disconnect_peer(DisconnectPeerRequest {
            pub_key: pubkey.to_string(),
        })
        .await;
}

pub async fn probe_peer(
    client: lnd_grpc_rust::LndClient,
    pubkey: &str,
) -> anyhow::Result<lightning_probing::ProbeResult> {
    let res = probe_destination({
        ProbeDestination {
            client,
            destination_pubkey: Some(pubkey.to_string()),
            fee_limit_sat: 100,
            last_hop_pubkey: None,
            max_paths: Some(1),
            outgoing_pubkeys: None,
            payment_request: None,
            probe_amount_sat: Some(1),
            timeout_seconds: Some(60),
        }
    })
    .await;

    return res;
}
