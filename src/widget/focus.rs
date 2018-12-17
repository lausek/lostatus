use std::time::Duration;

use i3ipc::event::Event::*;

use crate::config::{chars::CONTINUE, widget::focus::*};
use crate::i3::I3Output;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, UpdateEvent::*, Widget};

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
                let result = if let Some(name) = evt.container.name.clone() {
                    let output = if MAX_LENGTH < name.len() {
                        format!("{} {}", &name[..MAX_LENGTH - 1], CONTINUE)
                    } else {
                        name
                    };
                    Ok(I3Output::from_text(output.to_string()))
                } else {
                    Err("no focus")
                };
                Some((result, Some(INTERVAL)))
            }
            _ => None,
        }
    }
}
