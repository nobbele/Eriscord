use std::path::Path;

use eriscord::{
    async_trait,
    client::{self, Client},
    event::MessageCreateEvent,
    parser::Command,
    send_message, EventHandler,
};
use serde::{Deserialize, Serialize};

struct TestBot {}

impl TestBot {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait(?Send)]
impl EventHandler for TestBot {
    async fn on_command(
        &mut self,
        client: &Client,
        event: &MessageCreateEvent,
        command: Command<'_>,
    ) {
        match command.action {
            "ping" => send_message(client, &event.channel_id, "pong").await,
            _ => send_message(client, &event.channel_id, "Invalid command").await,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    token: String,
}

#[tokio::main]
async fn main() {
    let config_path = Path::new("./botconfig.toml");
    let config: Config = if config_path.exists() {
        let config_str = std::fs::read_to_string(config_path).unwrap();
        toml::from_str(&config_str).unwrap()
    } else {
        let config = Config {
            token: "YOUR TOKEN".to_owned(),
        };
        let config_str = toml::to_string_pretty(&config).unwrap();
        std::fs::write(config_path, config_str).unwrap();
        eprintln!(
            "Please enter the required information in {}",
            config_path.display()
        );
        std::process::exit(1);
    };
    let client = Client::new(config.token);
    let bot = TestBot::new();

    client::run(client, bot).await;
}
