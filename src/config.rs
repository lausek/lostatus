use std::time::Duration;

use crate::widget::*;

// TODO: make declaration nicer with procedural macro?
pub const SHELL: &str = "fish";
pub fn widgets() -> Vec<Box<dyn Widget>>
{
    vec![
        Box::new(Toggle::new()),
        Box::new(Focus::new()),
        Box::new(Battery::new()),
        Box::new(DateTime::new()),
    ]
}

pub mod chars
{
    pub const BRIGHTNESS: char = '☼';
    pub const BARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    pub const IO: &[char] = &['0', '1'];
}

pub mod app
{
    pub const INTERVAL: super::Duration = super::Duration::from_secs(4);
}

pub mod widget
{
    use std::time::Duration;

    pub mod battery
    {
        pub const INTERVAL: super::Duration = super::Duration::from_secs(60);
        pub const FILE_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
    }

    pub mod datetime
    {
        pub const INTERVAL: super::Duration = super::Duration::from_secs(30);
        pub const DATE_FORMAT: &str = "date +\"%H:%M / %d.%m.%Y\"";
    }

    pub mod focus
    {
        pub const INTERVAL: super::Duration = super::Duration::from_secs(60);
    }

    pub mod toggle
    {
        pub const INTERVAL: super::Duration = super::Duration::from_secs(30);
    }
}
