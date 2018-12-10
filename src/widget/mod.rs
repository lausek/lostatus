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
    // returns duration after which the widget wants its next update:
    //  - None: Block wasn't updated
    //  - Some: Block was updated and delivered some output      `BlockResult`:
    //          - Ok: New cache value of this block
    //          - Err: Error while updating block
    //      `Duration`:
    //          - Some: Update after this period of time
    //          - None: Don't schedule for updates anymore
    fn update(&mut self, evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>;
}
