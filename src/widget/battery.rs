use std::time::Duration;

use crate::output::I3Block;
use crate::widget::Widget;

const INTERVAL: Duration = Duration::from_secs(60);

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
    fn update(&mut self) -> Option<(I3Block, Duration)>
    {
        Some((block, INTERVAL))
    }
}
