use std::time::Duration;

use crate::output::I3Block;
use crate::widget::Widget;

pub struct Time {}

impl Time
{
    pub fn new() -> Self
    {
        Self {}
    }
}

impl Widget for Time
{
    fn update(&mut self, block: &mut I3Block) -> Option<Duration>
    {
        block.full_text = Some("das ist ein klasse text".to_string());

        None
    }
}
