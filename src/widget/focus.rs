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
        match evt {
            System(box WindowEvent(evt)) => {
                let block = I3Output::from_text(evt.container.name.clone().unwrap());
                Some((Ok(block), Some(INTERVAL)))
            }
            _ => None,
        }
    }
}
