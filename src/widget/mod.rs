mod battery;
mod datetime;
mod focus;

use std::time::Duration;

use crate::i3::{I3Input, I3Output};

pub use self::battery::Battery;
pub use self::datetime::DateTime;
pub use self::focus::Focus;

#[derive(Debug)]
pub enum UpdateEvent
{
    System(Box<i3ipc::event::Event>),
    User(I3Input),
    Time,
}

pub type BlockResult = Result<I3Output, &'static str>;

pub trait Widget
{
    fn needs_system(&self) -> bool
    {
        false
    }
    // returns duration after which the widget wants its next update
    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>;
}
