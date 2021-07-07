use async_trait::async_trait;
use client::Client;
use parser::Command;
use serde_json::json;

pub mod client;
pub mod event;
pub(crate) mod gateway;
pub mod parser;

#[async_trait(?Send)]
pub trait EventHandler: Sync {
    async fn on_message_create(&mut self, client: &Client, event: &event::MessageCreateEvent) {
        let command = {
            let parser = &client.inner.lock().unwrap().parser;
            parser.parse_command(&event.content)
        };
        if let Some(command) = command {
            self.on_command(client, event, command).await;
        }
    }
    async fn on_command(
        &mut self,
        _client: &Client,
        _event: &event::MessageCreateEvent,
        _command: Command<'_>,
    ) {
    }
}

pub async fn send_message(client: &Client, channel_id: &str, content: &str) {
    let client = &client.inner.lock().unwrap().web;
    let post = client.post(format!(
        "https://discord.com/api/v9/channels/{}/messages",
        channel_id
    ));
    post.json(&json!({ "content": content }))
        .send()
        .await
        .unwrap();
}
