use std::time::Duration;

use crate::config::widget::datetime::*;
use crate::i3::I3Output;
use crate::shell;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, Widget};

pub struct DateTime {}

impl DateTime
{
    pub fn new() -> Self
    {
        Self {}
    }
}

impl Widget for DateTime
{
    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        match *evt {
            UpdateEvent::Time => {
                let result = match shell(DATE_FORMAT) {
                    Ok(date) => {
                        let normalized = date.lines().next().unwrap();
                        Ok(I3Output::from_text(normalized))
                    }
                    _ => Err("date failed"),
                };
                Some((result, Some(INTERVAL)))
            }
            _ => None,
        }
    }
}
