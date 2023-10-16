use std::time::Instant;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{
    constants::{
        PEER_CONNECT_FAILURE_MESSAGE, PEER_CONNECT_SUCCESS_MESSAGE, PEER_CONNECT_WAIT_MESSAGE,
        PROBE_FAILURE_MESSAGE, PROBE_SUCCESS_MESSAGE, PROBE_WAIT_MESSAGE, WELCOME_MESSAGE,
    },
    get_lnd::get_lnd,
    utils::{connect_peer, probe_peer},
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Help Command")]
    Help,
    #[command(description = "Connect to a peer")]
    Connect(String),
    #[command(description = "Probe a peer")]
    Probe(String),
}

pub struct InitBot {
    pub client: lnd_grpc_rust::LndClient,
}

impl InitBot {
    pub async fn init(&self) {
        let bot = Bot::from_env();

        log::info!("Starting command bot...");
        let commands = Command::bot_commands();

        let _ = bot.set_my_commands(commands).await;

        Command::repl(bot, Self::answer).await;
    }

    async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Start => bot.send_message(msg.chat.id, WELCOME_MESSAGE).await?,
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::Connect(uri) => {
                let mut client = get_lnd().await.expect("failed to get client");

                bot.send_message(msg.chat.id, PEER_CONNECT_WAIT_MESSAGE)
                    .await?;

                let start = Instant::now();
                let connect_result = connect_peer(&mut client, &uri).await;
                let elapsed = start.elapsed().as_secs();

                let message = match connect_result {
                    Ok(_) => format!("{} {} seconds", PEER_CONNECT_SUCCESS_MESSAGE, elapsed),
                    Err(e) => {
                        log::error!("Failed to connect to peer {:?}", e);
                        format!(
                            "{} seconds: {:?}",
                            PEER_CONNECT_FAILURE_MESSAGE,
                            e.root_cause()
                        )
                    }
                };

                bot.send_message(msg.chat.id, message).await?
            }
            Command::Probe(pubkey) => {
                let client = get_lnd().await.expect("failed to get client");

                bot.send_message(msg.chat.id, PROBE_WAIT_MESSAGE).await?;

                let start = Instant::now();
                let probe_result = probe_peer(client, &pubkey).await;
                let elapsed = start.elapsed().as_secs();

                let message = match probe_result {
                    Ok(n) => {
                        log::info!("{} {} seconds", PROBE_SUCCESS_MESSAGE, elapsed);

                        if n.is_probe_success {
                            format!("{} {} seconds", PROBE_SUCCESS_MESSAGE, elapsed)
                        } else {
                            format!("{} {:?}", PROBE_FAILURE_MESSAGE, n.failure_reason)
                        }
                    }

                    Err(e) => {
                        log::error!("Failed to probe peer {:?}", e);
                        format!("{}: {:?}", PROBE_FAILURE_MESSAGE, e)
                    }
                };

                bot.send_message(msg.chat.id, format!("{:?}", message))
                    .await?
            }
        };

        Ok(())
    }
}
