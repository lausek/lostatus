use crate::config::widget::scroll::*;
use crate::i3::{I3Input, I3Output};
use crate::shell;
use crate::widget::{
    i3action::*, Action, BlockResult, BlockUpdateResult, UpdateEvent, UpdateEvent::*, Widget,
};

#[derive(Debug, Default)]
pub struct Scroll
{
    active: bool,
    cmd_up: Option<String>,
    cmd_down: Option<String>,
    cmd_status: Option<String>,
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

    fn exec(&self, cmd: &str) -> BlockResult
    {
        match shell(cmd) {
            Ok(content) => Ok(I3Output::from_text(format!("scroll: {}", content))),
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
