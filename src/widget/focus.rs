use super::*;

use i3ipc::event::Event::*;

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
                    let output = if FOCUS_MAX_LENGTH < name.len() {
                        format!("{} {}", &name[..FOCUS_MAX_LENGTH - 1], chars::CONTINUE)
                    } else {
                        name
                    };
                    Ok(Output::from_text(output))
                } else {
                    Err("no focus")
                };
                Some((result, Some(FOCUS_INTERVAL)))
            }
            _ => None,
        }
    }
}
