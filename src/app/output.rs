use super::*;

type Internal = Option<String>;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Output
{
    // TODO: this looks ugly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Internal,

    pub full_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_text: Internal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Internal,

    pub separator: bool,
    pub separator_block_width: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Internal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Internal,
}

impl Output
{
    pub fn new() -> Self
    {
        Self {
            color: Some(COLOR_SCHEME.basic.foreground.to_string()),
            background: Some(COLOR_SCHEME.basic.background.to_string()),
            ..Self::default()
        }
    }

    pub fn from_text<T>(full_text: T) -> Self
    where
        T: ToString,
    {
        let mut new = Self::new();
        new.full_text = full_text.to_string();
        new
    }
}
