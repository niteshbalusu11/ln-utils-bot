use std::time::Instant;

use anyhow::{bail, ensure};
use lightning_probing::{probe_destination, ProbeDestination};
use lnd_grpc_rust::lnrpc::{ConnectPeerRequest, DisconnectPeerRequest, LightningAddress};

use anyhow::Result;

use crate::constants::{
    PEER_CONNECT_FAILURE_MESSAGE, PEER_CONNECT_SUCCESS_MESSAGE, PROBE_FAILURE_MESSAGE,
    PROBE_SUCCESS_MESSAGE,
};

pub async fn get_connect_peer_message(
    client: &mut lnd_grpc_rust::LndClient,
    addr: &str,
) -> anyhow::Result<String> {
    let (pubkey, host) = parse_address(addr)?;

    disconnect_peer(client, &pubkey).await;

    log::info!("Connecting to peer {}", &pubkey);

    let start = Instant::now();

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
        .await;

    let elapsed = start.elapsed().as_secs();

    let message = match res {
        Ok(_) => format!("{} {} seconds", PEER_CONNECT_SUCCESS_MESSAGE, elapsed),
        Err(e) => {
            log::error!("Failed to connect to peer {:?}", e);
            format!("{} {:?}", PEER_CONNECT_FAILURE_MESSAGE, e)
        }
    };

    disconnect_peer(client, &pubkey).await;

    Ok(message)
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

pub async fn get_probe_peer_message(
    client: lnd_grpc_rust::LndClient,
    pubkey: &str,
) -> anyhow::Result<String> {
    if is_public_key(pubkey).is_err() || is_public_key(pubkey).unwrap() == false {
        bail!("ExpectedValidHexPublicKey".to_string());
    }

    let start = Instant::now();

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
            timeout_seconds: Some(20),
        }
    })
    .await;

    let elapsed = start.elapsed().as_secs();

    let message = match res {
        Ok(n) => {
            if n.is_probe_success {
                log::info!("{} {} seconds", PROBE_SUCCESS_MESSAGE, elapsed);
                format!("{} {} seconds", PROBE_SUCCESS_MESSAGE, elapsed)
            } else {
                log::info!("{} {:?}", PROBE_FAILURE_MESSAGE, n.failure_reason);
                format!("{} {:?}", PROBE_FAILURE_MESSAGE, n.failure_reason)
            }
        }
        Err(e) => {
            log::error!("Failed to probe peer {:?}", e);
            format!("{}: {:?}", PROBE_FAILURE_MESSAGE, e)
        }
    };

    return Ok(message);
}

fn is_public_key(n: &str) -> anyhow::Result<bool, regex::Error> {
    let re = regex::Regex::new(r"(?i)^0[2-3][0-9A-F]{64}$")?;
    Ok(re.is_match(n))
}
