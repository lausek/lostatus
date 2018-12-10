type Internal = Option<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct I3Input
{
    name: String,
    instance: String,

    button: i64,
    modifiers: Vec<String>,

    x: i64,
    y: i64,
    relative_x: i64,
    relative_y: i64,
    width: i64,
    height: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct I3Output
{
    // TODO: this looks ugly
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Internal,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Internal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_text: Internal,
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

            full_text: None,
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
