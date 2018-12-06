use std::time::Duration;

use crate::output::I3Block;
use crate::widget::Widget;

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
    fn update(&mut self, block: &mut I3Block) -> Option<Duration>
    {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time is messed up");
        let info = from_timestamp(timestamp);

        block.full_text = Some(format!("{:?}", info));

        Some(INTERVAL)
    }
}

fn from_timestamp(timestamp: Duration) -> (i64, i64)
{
    // TODO: implement unix timestamp calculation here
    (0, 0)
}
