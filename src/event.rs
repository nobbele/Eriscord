pub struct MessageAuthor {
    pub username: String,
    pub bot: bool,
}

pub struct MessageCreateEvent {
    pub author: MessageAuthor,
    pub channel_id: String,
    pub content: String,
}

pub struct InteractionCreateEvent {
    pub channel_id: String,
    pub custom_id: String,
    pub values: Vec<String>,
    pub(crate) interaction_token: String,
    pub(crate) interaction_id: String,
}

pub enum Event {
    MessageCreate(MessageCreateEvent),
    InteractionCreate(InteractionCreateEvent),
}
