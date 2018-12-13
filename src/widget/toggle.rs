use std::time::Duration;

use crate::i3::I3Output;
use crate::shell;
use crate::widget::{BlockResult, UpdateEvent, UpdateEvent::*, Widget, C_IO};

const INTERVAL: Duration = Duration::from_secs(30);

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
            User(_) if self.cmd.is_some() => match shell(self.cmd.as_ref().unwrap()) {
                Ok(_) => {
                    self.active = !self.active;
                    let icon = C_IO[if self.active { 1 } else { 0 }];
                    Ok(I3Output::from_text(format!("{}", icon)))
                }
                _ => Err("cmd failed"),
            },
            _ => return None,
        };

        Some((output, Some(INTERVAL)))
    }
}
