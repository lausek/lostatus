use std::time::Duration;

use crate::i3::I3Output;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, Widget};

const INTERVAL: Duration = Duration::from_secs(30);

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
    fn update(&mut self, _evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        let mut block = I3Output::default();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time is messed up");
        let info = from_timestamp(timestamp);

        block.full_text = format!("{:?}", info);

        Some((Ok(block), Some(INTERVAL)))
    }
}

fn from_timestamp(timestamp: Duration) -> (i64, i64)
{
    // TODO: implement unix timestamp calculation here
    (0, 0)
}
