use std::str::FromStr;
use std::time::Duration;

use crate::i3::I3Output;
use crate::widget::{BlockResult, UpdateEvent, Widget, C_BARS};

const INTERVAL: Duration = Duration::from_secs(60);
const FILE_PATH: &str = "/sys/class/power_supply/BAT0/capacity";

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
                    if let Some(content) = content.split_whitespace().next() {
                        match f64::from_str(content.as_ref()) {
                            Ok(capacity) => {
                                let mut block = I3Output::default();
                                let idx = (capacity / 101.0 * 9.0).floor() as usize;
                                block.full_text = format!("{}", C_BARS[idx]);
                                Ok(block)
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
