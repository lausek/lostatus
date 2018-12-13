use std::time::Duration;

use crate::i3::I3Output;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, UpdateEvent::*, Widget};

const INTERVAL: Duration = Duration::from_secs(30);

pub struct Toggle
{
    active: bool,
}

impl Toggle
{
    pub fn new() -> Self
    {
        Self { active: true }
    }
}

impl Widget for Toggle
{
    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        let mut block = I3Output::default();
        match evt {
            User(_) => self.active = !self.active,
            _ => {}
        }

        block.full_text = Some(format!("{}", self.active));

        Some((Ok(block), Some(INTERVAL)))
    }
}
