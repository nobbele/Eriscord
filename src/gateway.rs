use serde_json::Value;

use crate::event;

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationData {
    pub flags: u64,
    pub id: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GuildData {
    pub id: String,
    pub unavailable: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct UserData {
    pub id: String,
    pub avatar: String,
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub flags: u64,
    pub mfa_enabled: bool,
    pub username: String,
    pub verified: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct ReadyData {
    pub application: ApplicationData,
    pub geo_ordered_rtc_regions: Vec<String>,
    pub guild_join_requests: Value, // ?
    pub guilds: Vec<GuildData>,
    pub presences: Value,        // ?
    pub private_channels: Value, // ?
    pub relationships: Value,    // ?
    pub session_id: String,
    pub shard: Vec<u16>, // [shard_id, shard_count]
    pub user: UserData,
    pub user_settings: Value, // ?
    pub v: u8,
}

#[derive(serde::Deserialize, Debug)]
pub struct InitialData {
    pub heartbeat_interval: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct AuthorData {
    pub id: String,
    pub avatar: String,
    #[serde(default)]
    pub bot: bool,
    pub discriminator: String,
    pub public_flags: u64,
    pub username: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct MemberData {
    pub deaf: bool,
    pub hoisted_role: Option<String>,
    pub joined_at: String,
    pub mute: bool,
    pub roles: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreateMessageData {
    pub attachments: Vec<Value>, // ?
    pub author: AuthorData,
    pub channel_id: String,
    pub components: Vec<Value>, // ?
    pub content: String,
    pub edited_timestamp: Value, // ?
    pub embeds: Vec<Value>,      // ?
    pub flags: u64,
    pub guild_id: String,
    pub id: String,
    pub member: MemberData,
    pub mention_everyone: bool,
    pub mention_roles: Vec<Value>, // ?
    pub nonce: Option<String>,
    pub pinned: bool,
    pub referenced_message: Value, // ?
    pub timestamp: String,
    pub tts: bool,
    #[serde(alias = "type")]
    pub message_type: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct Response<T> {
    pub op: u64,
    #[serde(alias = "s")]
    pub seq: Option<u64>,
    #[serde(alias = "d")]
    pub data: T,
    #[serde(alias = "t")]
    pub event_name: Option<String>,
}

pub enum PacketData {
    Event(event::Event),
    HeartbeatAck,
}

pub struct Packet {
    pub seq: Option<u64>,
    pub data: PacketData,
}

#[derive(Debug)]
pub enum PacketParseError {
    UnknownPacket(u64),
    UnknownEvent(String),
}

pub fn parse_gateway_packet(text: String) -> Result<Packet, PacketParseError> {
    let resp: Response<Value> = serde_json::from_str(&text).unwrap();
    let packet = match resp.op {
        0 => {
            let event_name = resp.event_name.unwrap();
            let event = match event_name.as_str() {
                "MESSAGE_CREATE" => {
                    let data: CreateMessageData = serde_json::from_value(resp.data).unwrap();
                    event::Event::MessageCreate(event::MessageCreateEvent {
                        author: event::MessageAuthor {
                            username: data.author.username,
                            bot: data.author.bot,
                        },
                        channel_id: data.channel_id,
                        content: data.content,
                    })
                }
                name => return Err(PacketParseError::UnknownEvent(name.to_owned())),
            };
            PacketData::Event(event)
        }
        11 => PacketData::HeartbeatAck,
        n => return Err(PacketParseError::UnknownPacket(n)),
    };
    Ok(Packet {
        seq: resp.seq,
        data: packet,
    })
}
