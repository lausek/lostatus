type Internal = Option<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct I3Input
{
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub instance: String,

    #[serde(default)]
    pub button: i64,
    #[serde(default)]
    pub modifiers: Vec<String>,

    #[serde(default)]
    pub x: i64,
    #[serde(default)]
    pub y: i64,
    #[serde(default)]
    pub relative_x: i64,
    #[serde(default)]
    pub relative_y: i64,
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct I3Output
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_block_width: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Internal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Internal,
}

impl std::fmt::Display for I3Output
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", serde_json::to_string(self).unwrap())?;
        Ok(())
    }
}

impl Default for I3Output
{
    fn default() -> Self
    {
        Self {
            name: None,
            instance: None,

            full_text: String::new(),
            short_text: None,

            color: None,
            background: None,
            min_width: None,
            align: None,
            border: None,

            separator: None,
            separator_block_width: None,
            urgent: None,
            markup: None,
        }
    }
}
