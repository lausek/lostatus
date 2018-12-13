use std::time::Duration;

use i3ipc::event::Event::*;

use crate::i3::I3Output;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, UpdateEvent::*, Widget};

const INTERVAL: Duration = Duration::from_secs(60);

pub struct Focus {}

impl Focus
{
    pub fn new() -> Self
    {
        Self {}
    }
}

impl Widget for Focus
{
    fn needs_system(&self) -> bool
    {
        true
    }

    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        let mut block = I3Output::default();
        match evt {
            System(box WindowEvent(evt)) => block.full_text = evt.container.name.clone().unwrap(),
            _ => {}
        }
        Some((Ok(block), Some(INTERVAL)))
    }
}
