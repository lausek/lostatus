use std::str::FromStr;
use std::time::Duration;

use crate::config::{chars, widget::battery::*};
use crate::get_percentage_char;
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
            Ok(content) => Some((
                if let Some(content) = content.lines().next() {
                    match f64::from_str(content) {
                        Ok(capacity) => {
                            let symbol = get_percentage_char(capacity, &chars::BARS);
                            Ok(I3Output::from_text(format!("{}", symbol)))
                        }
                        _ => Err("invalid content"),
                    }
                } else {
                    Err("invalid capacity")
                },
                Some(INTERVAL),
            )),
            Err(_) => Some((Err("no battery"), None)),
        }
    }
}
