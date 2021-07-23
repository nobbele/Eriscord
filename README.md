# Eriscord
A discord library for the Rust programming language, designed to be easy to use.

## Example
```rust
/* EXCLUDING USE STATEMENTS */

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

#[tokio::main]
async fn main() {
    let client = Client::new("YOUR TOKEN");
    let bot = TestBot::new();

    client::run(client, bot).await;
}
```
