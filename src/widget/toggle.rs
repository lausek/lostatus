use super::*;

pub struct Toggle
{
    active: bool,
    cmd: Option<String>,
    fmt_callback: Box<Fn(&Self) -> Result<String, &'static str>>,
}

impl Toggle
{
    pub fn new() -> Self
    {
        Self {
            ..Default::default()
        }
    }

    pub fn formatter<T>(mut self, callback: T) -> Self
    where
        T: 'static + Fn(&Self) -> Result<String, &'static str>,
    {
        self.fmt_callback = Box::new(callback);
        self
    }

    pub fn command(mut self, cmd: &str) -> Self
    {
        self.cmd = Some(cmd.to_string());
        self
    }
}

impl Widget for Toggle
{
    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        let output = match evt {
            User(I3Input {
                button: I3_ACTION_LEFT,
                ..
            }) if self.cmd.is_some() => match shell(self.cmd.as_ref().unwrap()) {
                Ok(_) => {
                    self.active = !self.active;

                    build(
                        self.active,
                        (self.fmt_callback)(&self).unwrap_or(String::new()),
                    )
                }
                _ => Err("cmd failed"),
            },
            Time => build(
                self.active,
                (self.fmt_callback)(&self).unwrap_or(String::new()),
            ),
            _ => return None,
        };
        Some((output, Some(INTERVAL)))
    }
}

impl std::default::Default for Toggle
{
    fn default() -> Self
    {
        Self {
            active: true,
            cmd: None,
            fmt_callback: Box::new(|_| Ok(String::new())),
        }
    }
}

fn build(active: bool, msg: String) -> Result<I3Output, &'static str>
{
    let icon_from = |active: bool| chars::IO[if active { 1 } else { 0 }];

    let mut output = I3Output::from_text(format!("{} {}", icon_from(active), msg));

    if active {
        output.color = Some(COLOR_SCHEME.good.foreground.to_string());
        output.background = Some(COLOR_SCHEME.good.background.to_string());
    } else {
        output.color = Some(COLOR_SCHEME.bad.foreground.to_string());
        output.background = Some(COLOR_SCHEME.bad.background.to_string());
    }

    Ok(output)
}
