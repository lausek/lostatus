use std::str::FromStr;
use std::time::Duration;

use crate::i3::I3Output;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, Widget};

const INTERVAL: Duration = Duration::from_secs(60);
const FILE_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
const BAR_CHARS: &'static [char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

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
                let mut block = I3Output::default();

                block.full_text = Some(
                    match f64::from_str(content.as_ref()) {
                        Ok(capacity) => {
                            let idx = (capacity / 101.0 * 9.0).floor() as usize;
                            format!("{}", BAR_CHARS[idx])
                        },
                        Err(_) => "invalid capacity".to_string(),
                    }
                );

                Some((Ok(block), Some(INTERVAL)))
            }
            Err(_) => Some((Err("no battery"), None)),
        }
    }
}
