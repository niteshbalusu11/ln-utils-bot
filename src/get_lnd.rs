use std::{env, fs};

pub async fn get_lnd() -> anyhow::Result<lnd_grpc_rust::LndClient> {
    let socket = get_socket();

    let cert = get_cert();
    let macaroon = get_macaroon();

    let client = lnd_grpc_rust::connect(cert, macaroon, socket)
        .await
        .expect("Failed to get LND Client");

    return Ok(client);
}

fn get_from_env_or_file(env_var: &str, file_var: &str, error_msg: &str) -> String {
    let path = env::var(file_var).unwrap_or_default();
    let hex_value = env::var(env_var).unwrap_or_default();

    if path.is_empty() && hex_value.is_empty() {
        panic!("{}", error_msg);
    }

    if !path.is_empty() {
        let bytes = fs::read(&path).expect(&format!("FailedToReadFile: {}", path));
        hex::encode(bytes)
    } else {
        hex_value
    }
}

fn get_cert() -> String {
    get_from_env_or_file(
        "CERT_HEX",
        "CERT_PATH",
        "ExpectedEitherTlsCertPathOrTlsCertHexToAuthenticateToLnd",
    )
}

fn get_macaroon() -> String {
    get_from_env_or_file(
        "MACAROON_HEX",
        "MACAROON_PATH",
        "ExpectedEitherMacaroonPathOrMacaroonHexToAuthenticateToLnd",
    )
}

fn get_socket() -> String {
    env::var("SOCKET").expect("ExpectedSocketToAuthenticateToLnd")
}
