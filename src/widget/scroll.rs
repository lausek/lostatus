use crate::config::{chars, widget::scroll::*};
use crate::i3::{I3Input, I3Output};
use crate::shell;
use crate::widget::{
    i3action::*, BlockResult, BlockUpdateResult, MouseAction, UpdateEvent, UpdateEvent::*, Widget,
};

#[derive(Debug, Default)]
pub struct Scroll
{
    active: bool,
    icon: String,
    cmd_up: Option<String>,
    cmd_down: Option<String>,
}

impl Scroll
{
    pub fn new() -> Self
    {
        Self {
            ..Default::default()
        }
    }

    pub fn icon(mut self, icon: &str) -> Self
    {
        self.icon = icon.to_string();
        self
    }

    pub fn command(mut self, on: MouseAction, cmd: &str) -> Self
    {
        match on {
            MouseAction::ScrollUp => {
                self.cmd_up = Some(cmd.to_string());
            }
            MouseAction::ScrollDown => {
                self.cmd_down = Some(cmd.to_string());
            }
            _ => {}
        }
        self
    }

    fn exec(&self, cmd: &str) -> BlockResult
    {
        match shell(cmd) {
            Ok(_) => Ok(I3Output::from_text(format!("{}", self.icon))),
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
            _ => return None,
        };

        Some((output, Some(INTERVAL)))
    }
}
