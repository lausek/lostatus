mod datetime;

use std::time::Duration;

use crate::output::I3Block;

pub use self::datetime::DateTime;

pub trait Widget
{
    // returns duration after which the widget wants its next update
    fn update(&mut self, block: &mut I3Block) -> Option<Duration>;
}
