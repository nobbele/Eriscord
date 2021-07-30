use std::path::Path;

use eriscord::{
    async_trait,
    client::{self, Client},
    event::{InteractionCreateEvent, MessageCreateEvent},
    message::{Component, ComponentType, Message, SelectOption},
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
            "test" => {
                send_message(
                    client,
                    &event.channel_id,
                    Message::new().with_text("Hello World").with_new_component(
                        Component::new()
                            .with_component_type(ComponentType::SelectMenu)
                            .with_custom_id("select_one")
                            .with_new_option(
                                SelectOption::new().with_label("Test 1").with_value("val1"),
                            )
                            .with_new_option(
                                SelectOption::new().with_label("Test 2").with_value("val2"),
                            ),
                    ),
                )
                .await
            }
            _ => send_message(client, &event.channel_id, "Invalid command").await,
        }
    }
    async fn on_interaction(
        &mut self,
        _client: &Client,
        event: &InteractionCreateEvent,
    ) -> Option<Message> {
        Some(
            match event.custom_id.as_str() {
                "select_one" => match event.values[0].as_str() {
                    "val1" => "You chose the option containing 'val1'",
                    "val2" => "You chose the option containing 'val2'",
                    _ => "Unknown",
                },
                _ => "Unknown",
            }
            .into(),
        )
    }
}

#[derive(Deserialize, Serialize)]
struct Config {
    token: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Self {
        let config_path = Path::new(path);
        if config_path.exists() {
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
        }
    }
}

#[tokio::main]
async fn main() {
    let config = Config::load_from_file("./botconfig.toml");
    let client = Client::new(config.token);
    let bot = TestBot::new();

    client::run(client, bot).await;
}
