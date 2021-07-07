pub struct MessageAuthor {
    pub username: String,
    pub bot: bool,
}

pub struct MessageCreateEvent {
    pub author: MessageAuthor,
    pub channel_id: String,
    pub content: String,
}

pub enum Event {
    MessageCreate(MessageCreateEvent),
}
