use super::*;

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

    pub separator: bool,
    pub separator_block_width: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urgent: Internal,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub markup: Internal,
}

impl I3Output
{
    pub fn from_text<T>(full_text: T) -> Self
    where
        T: ToString,
    {
        Self {
            full_text: full_text.to_string(),
            ..Self::default()
        }
    }
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

            color: Some(COLOR_SCHEME.basic.foreground.to_string()),
            background: Some(COLOR_SCHEME.basic.background.to_string()),
            min_width: None,
            align: None,
            border: None,

            separator: false,
            separator_block_width: 0,

            urgent: None,
            markup: None,
        }
    }
}

macro_rules! i3print {
    ($msg:expr, $($x:expr),*) => {{ i3print!(format!($msg, $($x),*)); }};
    ($msg:expr) => {{ print!("{}", $msg); }};
}

macro_rules! i3flush {
    () => {{
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }};
}

macro_rules! i3error {
    ($msg:expr) => {
        // TODO: add color
        format!("{{\"full_text\": \"{}\"}}", $msg)
    };
}

#[cfg(feature = "i3")]
pub fn output_init()
{
    i3print!("{ \"version\": 1, \"click_events\": true } [");
    i3flush!();
}

#[cfg(feature = "i3")]
pub fn output_render(app: &App)
{
    i3print!("\n[");

    let mut iter = app.widgets.iter();
    let separator = I3Output::from_text("|");

    if let Some((ref first, _)) = iter.next() {
        i3print!(first);
    }

    for (ref block, _) in iter {
        i3print!(",{}", separator);
        i3print!(",{}", block);
    }

    i3print!("],");
    i3flush!();
}

#[cfg(feature = "i3")]
pub fn output_error(msg: &str) -> String
{
    i3error!(msg)
}

#[cfg(feature = "i3")]
pub fn spawn_system_sender(sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    let mut i3 = i3ipc::I3EventListener::connect().expect("i3 not running");

    Some(thread::spawn(move || {
        i3.subscribe(&[i3ipc::Subscription::Window])
            .expect("could not subscribe to i3 events");

        // TODO: does listen block?
        loop {
            for event in i3.listen() {
                match event {
                    Ok(event) => {
                        debug_log!("from system");

                        let sys_event = UpdateEvent::System(Box::new(event));
                        sender.send(sys_event).unwrap();
                    }
                    _ => panic!("system event is err"),
                }
            }
        }
    }))
}
