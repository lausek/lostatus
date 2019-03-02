use super::*;

pub struct Scroll
{
    pub active: bool,
    pub cmd_up: Option<String>,
    pub cmd_down: Option<String>,
    pub cmd_status: Option<String>,
    fmt_callback: Box<Fn(&Self) -> Result<String, &'static str>>,
}

impl Scroll
{
    pub fn new() -> Self
    {
        Self {
            ..Default::default()
        }
    }

    pub fn command(mut self, on: Action, cmd: &str) -> Self
    {
        match on {
            Action::ScrollUp => self.cmd_up = Some(cmd.to_string()),
            Action::ScrollDown => self.cmd_down = Some(cmd.to_string()),
            Action::Status => self.cmd_status = Some(cmd.to_string()),
            _ => {}
        }
        self
    }

    pub fn formatter<T>(mut self, callback: T) -> Self
    where
        T: 'static + Fn(&Self) -> Result<String, &'static str>,
    {
        self.fmt_callback = Box::new(callback);
        self
    }

    fn exec(&self, cmd: &str) -> BlockResult
    {
        match shell(cmd) {
            Ok(_content) => match (self.fmt_callback)(&self) {
                Ok(status) => Ok(I3Output::from_text(status)),
                Err(msg) => Err(msg),
            },
            _ => Err("cmd failed"),
        }
    }
}

impl Widget for Scroll
{
    fn update(&mut self, evt: &UpdateEvent) -> BlockUpdateResult
    {
        let output = match evt {
            User(I3Input { button, .. }) => match *button {
                I3_ACTION_SCROLL_UP => {
                    if let Some(cmd) = &self.cmd_up {
                        self.exec(cmd.as_ref())
                    } else {
                        return None;
                    }
                }
                I3_ACTION_SCROLL_DOWN => {
                    if let Some(cmd) = &self.cmd_down {
                        self.exec(cmd.as_ref())
                    } else {
                        return None;
                    }
                }
                _ => return None,
            },
            Time => {
                if let Some(cmd) = &self.cmd_status {
                    self.exec(cmd.as_ref())
                } else {
                    Err("no status cmd")
                }
            }
            _ => return None,
        };

        Some((output, Some(INTERVAL)))
    }
}

impl std::default::Default for Scroll
{
    fn default() -> Self
    {
        Self {
            active: false,
            cmd_up: None,
            cmd_down: None,
            cmd_status: None,
            fmt_callback: Box::new(|state| {
                if let Some(cmd) = &state.cmd_status {
                    return match shell(cmd.as_ref()) {
                        Ok(content) => {
                            let status = content
                                .lines()
                                .next()
                                .expect("error on fetching cmd status");
                            Ok(status.to_string())
                        }
                        _ => Err("cmd failed"),
                    };
                }
                Err("no status cmd")
            }),
        }
    }
}
