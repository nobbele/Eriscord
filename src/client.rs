use std::{
    sync::{Arc, RwLock},
    time::Duration,
};

use futures::{SinkExt, StreamExt};
use reqwest::header;
use serde_json::json;
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::{http, Message},
};

use crate::{
    event,
    gateway::{self, parse_gateway_packet, PacketData},
    parser::CommandParser,
    EventHandler,
};

pub(crate) struct ClientRaw {
    pub(crate) web: reqwest::Client,
    pub(crate) parser: CommandParser,
    pub(crate) seq: RwLock<Option<u64>>,
    pub(crate) token: String,
}

#[derive(Clone)]
pub struct Client {
    pub(crate) inner: Arc<ClientRaw>,
}

impl Client {
    pub fn new(token: String) -> Self {
        let web = {
            let mut headers = header::HeaderMap::new();
            let mut header_value =
                header::HeaderValue::from_str(&format!("Bot {}", token)).unwrap();
            header_value.set_sensitive(true);
            headers.insert("Authorization", header_value);
            reqwest::Client::builder()
                .default_headers(headers)
                .timeout(Duration::from_millis(1000))
                .build()
                .unwrap()
        };

        let raw = ClientRaw {
            seq: RwLock::new(None),
            web,
            parser: CommandParser::new(".".to_owned()),
            token,
        };

        Client {
            inner: Arc::new(raw),
        }
    }
}

pub async fn run<T: EventHandler>(client: Client, mut bot: T) -> ! {
    let gateway_url = {
        #[derive(serde::Deserialize, Debug)]
        struct GetResponse {
            url: String,
        }

        let response = client
            .inner
            .web
            .get("https://discord.com/api/v9/gateway")
            .send()
            .await
            .unwrap();
        let body = response.json::<GetResponse>().await.unwrap();

        body.url
    };

    {
        let request = http::Request::builder()
            .uri(format!("{}/?v=9&encoding=json", gateway_url))
            .body(())
            .unwrap();

        let (ws, _resp) = connect_async_with_config(request, None).await.unwrap();
        let (mut write, mut read) = ws.split();

        println!("Connected!");

        let message = read.next().await.unwrap().unwrap();
        let heartbeat_interval = read_hello_message(&client, message).unwrap();

        println!(
            "Heartbeat Interval set to {}ms",
            heartbeat_interval.as_millis()
        );

        {
            let json = json!({
                "op": 2,
                "d": {
                    "token": client.inner.token,
                    "properties": {
                        "$os": "Windows 10",
                        "$browser": "foo",
                        "$device": "bar"
                    },
                    "compress": false,
                    "large_threshold": 50,
                    "shard": [0, 1],
                    "presence": {
                    "activities": [{
                        "name": "Bot stuff",
                        "type": 0
                    }],
                    "status": "online",
                    "since": null,
                    "afk": false
                    },
                    "intents": (1 << 9) | (1 << 13) // Message + Reaction
                }
            });
            let json_str = serde_json::to_string(&json).unwrap();
            println!("Sending Identify");
            write.send(Message::Text(json_str)).await.unwrap();
        }

        let _ready_data = {
            let message = read.next().await.expect("WebSocket Disconnected").unwrap();
            read_ready_message(&client, message).unwrap()
        };

        let _heartbeat_task = tokio::spawn({
            let client = client.inner.clone();
            async move {
                let mut interval = tokio::time::interval(Duration::from_millis(
                    (heartbeat_interval.as_millis() as f32 * 0.95) as _, // Send it 5% early for some margin of error
                ));

                loop {
                    interval.tick().await;
                    let json = json!({
                        "op": 1,
                        "d": client.seq
                    });
                    let json_str = serde_json::to_string(&json).unwrap();
                    println!("Sending Heartbeat");
                    write.send(Message::Text(json_str)).await.unwrap();
                }
            }
        });

        loop {
            let message = read.next().await.expect("WebSocket Disconnected");
            match message {
                Ok(Message::Text(text)) => {
                    let packet = parse_gateway_packet(text).unwrap();
                    if let Some(seq) = packet.seq {
                        *client.inner.seq.write().unwrap() = Some(seq);
                    }
                    match packet.data {
                        PacketData::Event(event) => match event {
                            event::Event::MessageCreate(event) => {
                                bot.on_message_create(&client, &event).await;
                            }
                        },
                        PacketData::HeartbeatAck => println!("Heartbeat Acknowledged"),
                    };
                }
                Ok(msg) => eprintln!("Non-Text Message: {:?}", msg),
                Err(e) => panic!("Error: {}", e),
            }
        }
    }
}

fn read_hello_message(client: &Client, msg: Message) -> Result<Duration, ()> {
    if let Message::Text(text) = msg {
        let value: gateway::Response<gateway::InitialData> = serde_json::from_str(&text).unwrap();

        assert_eq!(value.op, 10);

        if let Some(seq) = value.seq {
            *client.inner.seq.write().unwrap() = Some(seq);
        }

        return Ok(Duration::from_millis(value.data.heartbeat_interval));
    }
    Err(())
}

fn read_ready_message(client: &Client, msg: Message) -> Result<gateway::ReadyData, ()> {
    if let Message::Text(text) = msg {
        let value: gateway::Response<gateway::ReadyData> = serde_json::from_str(&text).unwrap();

        assert_eq!(value.op, 0);

        if let Some(seq) = value.seq {
            *client.inner.seq.write().unwrap() = Some(seq);
        }

        return Ok(value.data);
    }
    Err(())
}
