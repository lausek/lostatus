mod battery;
mod datetime;
mod focus;
mod scroll;
mod toggle;

use super::*;

pub use crate::{app::*, config::*};

pub use self::battery::Battery;
pub use self::datetime::DateTime;
pub use self::focus::Focus;
pub use self::scroll::Scroll;
pub use self::toggle::Toggle;
pub use self::UpdateEvent::*;

pub use std::time::Duration;

pub const I3_ACTION_LEFT: i64 = 1;
pub const I3_ACTION_MIDDLE: i64 = 2;
pub const I3_ACTION_RIGHT: i64 = 3;
pub const I3_ACTION_SCROLL_UP: i64 = 4;
pub const I3_ACTION_SCROLL_DOWN: i64 = 5;

pub type BlockResult = Result<Output, &'static str>;
pub type BlockUpdateResult = Option<(BlockResult, Option<Duration>)>;

#[derive(Debug)]
pub enum UpdateEvent
{
    System(Box<i3ipc::event::Event>),
    User(Input),
    Time,
}

#[derive(Debug, PartialEq)]
pub enum Action
{
    Left,
    Middle,
    Right,
    ScrollUp,
    ScrollDown,
    Status,
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
