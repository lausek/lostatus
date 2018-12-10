use std::time::Duration;

use crate::i3::I3Output;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, Widget};

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
    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        match std::fs::read_to_string(FILE_PATH) {
            Ok(content) => {
                let mut block = I3Output::default();

                block.full_text = Some(content);

                Some((Ok(block), Some(INTERVAL)))
            }
            Err(_) => Some((Err("no battery"), None)),
        }
    }
}
