// TODO macro for all this

#[derive(serde::Serialize, Debug, Default)]
pub struct Footer<'a> {
    text: Option<&'a str>,
    icon_url: Option<&'a str>,
    proxy_icon_url: Option<&'a str>,
}

impl<'a> Footer<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    #[must_use]
    pub fn with_icon_url(mut self, icon_url: &'a str) -> Self {
        self.icon_url = Some(icon_url);
        self
    }

    #[must_use]
    pub fn with_proxy_icon_url(mut self, proxy_icon_url: &'a str) -> Self {
        self.proxy_icon_url = Some(proxy_icon_url);
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Image<'a> {
    url: Option<&'a str>,
    proxy_url: Option<&'a str>,
    height: Option<u32>,
    width: Option<u32>,
}

impl<'a> Image<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    #[must_use]
    pub fn with_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.proxy_url = Some(proxy_url);
        self
    }

    #[must_use]
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    #[must_use]
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Thumbnail<'a> {
    url: Option<&'a str>,
    proxy_url: Option<&'a str>,
    height: Option<u32>,
    width: Option<u32>,
}

impl<'a> Thumbnail<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    #[must_use]
    pub fn with_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.proxy_url = Some(proxy_url);
        self
    }

    #[must_use]
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    #[must_use]
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Video<'a> {
    url: Option<&'a str>,
    proxy_url: Option<&'a str>,
    height: Option<u32>,
    width: Option<u32>,
}

impl<'a> Video<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    #[must_use]
    pub fn with_proxy_url(mut self, proxy_url: &'a str) -> Self {
        self.proxy_url = Some(proxy_url);
        self
    }

    #[must_use]
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    #[must_use]
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Provider<'a> {
    name: Option<&'a str>,
    url: Option<&'a str>,
}

impl<'a> Provider<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub fn with_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Author<'a> {
    name: Option<&'a str>,
    url: Option<&'a str>,
    icon_url: Option<&'a str>,
    proxy_icon_url: Option<&'a str>,
}

impl<'a> Author<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub fn with_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    #[must_use]
    pub fn with_icon_url(mut self, icon_url: &'a str) -> Self {
        self.icon_url = Some(icon_url);
        self
    }

    #[must_use]
    pub fn with_proxy_icon_url(mut self, proxy_icon_url: &'a str) -> Self {
        self.proxy_icon_url = Some(proxy_icon_url);
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Field<'a> {
    name: Option<&'a str>,
    value: Option<&'a str>,
    inline: bool,
}

impl<'a> Field<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    #[must_use]
    pub fn with_value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    #[must_use]
    pub fn with_inline(mut self) -> Self {
        self.inline = true;
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Embed<'a> {
    title: Option<&'a str>,
    description: Option<&'a str>,
    url: Option<&'a str>,
    timestamp: Option<()>, // ? type
    color: Option<u32>,    // TODO color struct
    footer: Option<Footer<'a>>,
    image: Option<Image<'a>>,
    thumbnail: Option<Thumbnail<'a>>,
    video: Option<Video<'a>>,
    provider: Option<Provider<'a>>,
    author: Option<Author<'a>>,
    fields: Vec<Field<'a>>,
}

impl<'a> Embed<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    #[must_use]
    pub fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    #[must_use]
    pub fn with_url(mut self, url: &'a str) -> Self {
        self.url = Some(url);
        self
    }

    #[must_use] // TODO
    pub fn with_timestamp(mut self, timestamp: ()) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    #[must_use]
    pub fn with_color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    #[must_use]
    pub fn with_footer(mut self, footer: Footer<'a>) -> Self {
        self.footer = Some(footer);
        self
    }

    #[must_use]
    pub fn with_image(mut self, image: Image<'a>) -> Self {
        self.image = Some(image);
        self
    }

    #[must_use]
    pub fn with_thumbnail(mut self, thumbnail: Thumbnail<'a>) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    #[must_use]
    pub fn with_video(mut self, video: Video<'a>) -> Self {
        self.video = Some(video);
        self
    }

    #[must_use]
    pub fn with_provider(mut self, provider: Provider<'a>) -> Self {
        self.provider = Some(provider);
        self
    }

    #[must_use]
    pub fn with_author(mut self, author: Author<'a>) -> Self {
        self.author = Some(author);
        self
    }

    #[must_use]
    pub fn with_fields<const N: usize>(mut self, fields: [Field<'a>; N]) -> Self {
        self.fields.extend(fields);
        self
    }

    pub fn with_new_field(mut self, field: Field<'a>) -> Self {
        self.fields.push(field);
        self
    }

    pub fn add_field(&mut self, field: Field<'a>) {
        self.fields.push(field);
    }
}

#[derive(serde_repr::Serialize_repr, Debug)]
#[repr(u32)]
pub enum ComponentType {
    ActionRow = 1,
    Button = 2,
    SelectMenu = 3,
}

impl Default for ComponentType {
    fn default() -> Self {
        Self::ActionRow
    }
}

#[derive(serde_repr::Serialize_repr, Debug)]
#[repr(u32)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::Primary
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct SelectOption<'a> {
    label: Option<&'a str>,
    value: Option<&'a str>,
    description: Option<&'a str>,
    emoji: Option<()>, // TODO
    default: Option<bool>,
}

impl<'a> SelectOption<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    #[must_use]
    pub fn with_value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }
}

// TODO enum
#[derive(serde::Serialize, Debug, Default)]
pub struct Component<'a> {
    #[serde(rename = "type")]
    component_type: ComponentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<()>, // TODO,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    options: Vec<SelectOption<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_values: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_values: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    components: Vec<Component<'a>>,
}

impl<'a> Component<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_component_type(mut self, component_type: ComponentType) -> Self {
        self.component_type = component_type;
        self
    }

    #[must_use]
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    #[must_use]
    pub fn with_custom_id(mut self, custom_id: &'a str) -> Self {
        self.custom_id = Some(custom_id);
        self
    }

    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    #[must_use]
    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    #[must_use]
    pub fn with_new_option(mut self, option: SelectOption<'a>) -> Self {
        self.options.push(option);
        self
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct Message<'a> {
    #[serde(rename = "content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embed: Option<Embed<'a>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    components: Vec<Component<'a>>,
}

impl<'a> Message<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_text(mut self, text: &'a str) -> Self {
        self.text = Some(text);
        self
    }

    #[must_use]
    pub fn with_embed(mut self, embed: Embed<'a>) -> Self {
        self.embed = Some(embed);
        self
    }

    #[must_use]
    pub fn with_component_row(mut self, components: Vec<Component<'a>>) -> Self {
        self.components = components;
        self
    }

    #[must_use]
    pub fn with_new_component(mut self, component: Component<'a>) -> Self {
        let top_level = if self.components.len() == 1 {
            &mut self.components[0]
        } else if self.components.len() == 0 {
            self.components.push(Component::new());
            &mut self.components[0]
        } else {
            panic!("This function can't be used with multi-level components");
        };
        top_level.components.push(component);
        self
    }
}

impl<'a> From<&'a str> for Message<'a> {
    fn from(s: &'a str) -> Self {
        Message::default().with_text(s)
    }
}

impl<'a> From<&'a String> for Message<'a> {
    fn from(s: &'a String) -> Self {
        Message::from(s as &str)
    }
}

impl<'a> From<Embed<'a>> for Message<'a> {
    fn from(embed: Embed<'a>) -> Self {
        Message::default().with_embed(embed)
    }
}
