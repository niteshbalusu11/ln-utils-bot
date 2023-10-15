use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{
    get_lnd::get_lnd,
    utils::{connect_peer, probe_peer},
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
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

        Command::repl(bot, Self::answer).await;
    }

    async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        let mut client = get_lnd().await.expect("failed to get client");

        match cmd {
            Command::Help => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::Connect(uri) => {
                let message = match connect_peer(&mut client, &uri).await {
                    Ok(_) => "Successfully connected to peer".to_string(),
                    Err(e) => {
                        log::error!("Failed to connect to peer {:?}", e);
                        format!("Failed to connect to peer {:?}", e)
                    }
                };

                bot.send_message(msg.chat.id, message).await?
            }
            Command::Probe(pubkey) => {
                let message = match probe_peer(client, &pubkey).await {
                    Ok(n) => {
                        log::info!("Successfullt probed peer");

                        if n.is_probe_success {
                            format!("Successfully probed peer")
                        } else {
                            format!("Failed to probe peer {:?}", n.failure_reason)
                        }
                    }

                    Err(e) => {
                        log::error!("Failed to probe peer {:?}", e);
                        format!("Failed to probe peer {:?}", e)
                    }
                };

                bot.send_message(msg.chat.id, format!("{:?}", message))
                    .await?
            }
        };

        Ok(())
    }
}
