use async_trait::async_trait;
use eriscord::{
    client::{self, Client},
    event::MessageCreateEvent,
    parser::Command,
    send_message, EventHandler,
};

const TOKEN: &str = "YOUR TOKEN";

struct TestBot {
    n: u32,
}

impl TestBot {
    pub fn new() -> Self {
        Self { n: 0 }
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
            "ping" => {
                send_message(client, &event.channel_id, &format!("pong {}", self.n)).await;
                self.n += 1;
            }
            "echo" => {
                let args = command.unparsed_args.parse(true);
                send_message(client, &event.channel_id, &args.text.unwrap()).await;
            }
            _ => {
                send_message(client, &event.channel_id, "Invalid command").await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new(TOKEN.to_owned());
    let bot = TestBot::new();

    client::run(client, bot).await;
}
