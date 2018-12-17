use std::time::Duration;

use crate::config::{chars, widget::toggle::*};
use crate::i3::{I3Input, I3Output};
use crate::shell;
use crate::widget::{i3action::*, BlockResult, UpdateEvent, UpdateEvent::*, Widget};

#[derive(Debug, Default)]
pub struct Toggle
{
    active: bool,
    cmd: Option<String>,
}

impl Toggle
{
    pub fn new() -> Self
    {
        Self {
            ..Default::default()
        }
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
                    let icon = chars::IO[if self.active { 1 } else { 0 }];
                    Ok(I3Output::from_text(format!("{}", icon)))
                }
                _ => Err("cmd failed"),
            },
            _ => return None,
        };
        Some((output, Some(INTERVAL)))
    }
}
