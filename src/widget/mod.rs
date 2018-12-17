mod battery;
mod datetime;
mod focus;
mod scroll;
mod toggle;

use std::time::Duration;

use crate::i3::{I3Input, I3Output};

pub use self::battery::Battery;
pub use self::datetime::DateTime;
pub use self::focus::Focus;
pub use self::scroll::Scroll;
pub use self::toggle::Toggle;

pub mod i3action
{
    pub const I3_ACTION_LEFT: i64 = 1;
    pub const I3_ACTION_RIGHT: i64 = 2;
    pub const I3_ACTION_SCROLL_UP: i64 = 3;
    pub const I3_ACTION_SCROLL_DOWN: i64 = 4;
}

pub type BlockResult = Result<I3Output, &'static str>;
pub type BlockUpdateResult = Option<(BlockResult, Option<Duration>)>;

#[derive(Debug)]
pub enum UpdateEvent
{
    System(Box<i3ipc::event::Event>),
    User(I3Input),
    Time,
}

#[derive(Debug, PartialEq)]
pub enum MouseAction
{
    Left,
    Right,
    ScrollUp,
    ScrollDown,
}

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
    fn update(&mut self, evt: &UpdateEvent) -> BlockUpdateResult;
}
