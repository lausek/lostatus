use std::str::FromStr;
use std::time::Duration;

use crate::config::{chars, widget::battery::*};
use crate::i3::I3Output;
use crate::widget::{BlockResult, UpdateEvent, Widget};

pub struct Battery {}

impl Battery
{
    pub fn new() -> Self
    {
        Self {}
    }
}

impl Widget for Battery
{
    fn update(&mut self, _evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        // TODO: only do timed updates here
        match std::fs::read_to_string(FILE_PATH) {
            Ok(content) => {
                let result = {
                    if let Some(content) = content.lines().next() {
                        match f64::from_str(content) {
                            Ok(capacity) => {
                                let idx =
                                    (capacity / 101.0 * chars::BARS.len() as f64).floor() as usize;
                                Ok(I3Output::from_text(format!("{}", chars::BARS[idx])))
                            }
                            _ => Err("invalid capacity"),
                        }
                    } else {
                        Err("invalid capacity")
                    }
                };
                Some((result, Some(INTERVAL)))
            }
            Err(_) => Some((Err("no battery"), None)),
        }
    }
}
