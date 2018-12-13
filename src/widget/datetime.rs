use std::time::Duration;

use crate::i3::I3Output;
use crate::shell;
use crate::widget::BlockResult;
use crate::widget::{UpdateEvent, Widget};

const INTERVAL: Duration = Duration::from_secs(30);
const DATE_FORMAT: &str = "date +\"%H:%M / %d.%m.%Y\"";

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
    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        match evt {
            &UpdateEvent::Time => {
                let result = match shell(DATE_FORMAT) {
                    Ok(date) => Ok(I3Output::from_text(format!("{:?}", date))),
                    _ => Err("date failed"),
                };
                Some((result, Some(INTERVAL)))
            }
            _ => None,
        }
    }
}
