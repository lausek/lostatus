use std::str::FromStr;
use std::time::Duration;

use crate::get_percentage_char;
use crate::widget::*;

// TODO: make declaration nicer with procedural macro?
pub const SHELL: &str = "fish";
pub fn widgets() -> Vec<Box<dyn Widget>>
{
    let volume = Scroll::new()
        .command(Action::ScrollUp, "~/.config/scripts/volume up")
        .command(Action::ScrollDown, "~/.config/scripts/volume down")
        .command(Action::Status, "~/.config/scripts/volume")
        .formatter(|state| {
            state
                .cmd_status
                .as_ref()
                .map_or(Err("no status cmd"), |cmd| {
                    match crate::shell(cmd.as_ref()) {
                        Ok(output) => {
                            let status = output.lines().next().unwrap().to_string();
                            let icon = match f64::from_str(status.as_ref()) {
                                Ok(volume) => get_percentage_char(volume, &chars::VOLUME),
                                _ => panic!("could not interpret status command as number"),
                            };
                            Ok(format!("{} {}", icon, status))
                        }
                        Err(_) => Err("status command failed"),
                    }
                })
        });

    // TODO: this blog is way too slow
    let brightness = Scroll::new()
        .command(Action::ScrollUp, "~/.config/scripts/brightness-lostatus +5")
        .command(
            Action::ScrollDown,
            "~/.config/scripts/brightness-lostatus -5",
        )
        .command(Action::Status, "~/.config/scripts/brightness-lostatus")
        .formatter(|state| {
            state
                .cmd_status
                .as_ref()
                .map_or(Err("no status cmd"), |cmd| {
                    match crate::shell(cmd.as_ref()) {
                        Ok(output) => {
                            let status = output.lines().next().unwrap().to_string();
                            let icon = match f64::from_str(status.as_ref()) {
                                Ok(volume) => get_percentage_char(volume, &chars::VOLUME),
                                _ => panic!("could not interpret status command as number"),
                            };
                            Ok(format!("{} {}", icon, status))
                        }
                        Err(_) => Err("status command failed"),
                    }
                })
        });

    let headset = Toggle::new().command("~/.config/scripts/headset-switch-toggle");

    vec![
        Box::new(Focus::new()),
        //Box::new(brightness),
        Box::new(volume),
        Box::new(headset),
        Box::new(Battery::new()),
        Box::new(DateTime::new()),
    ]
}

pub mod chars
{
    pub const BRIGHTNESS: char = '☼';
    pub const CONTINUE: char = '\u{2026}';
    pub const VOLUME: &[char] = &['\u{f00d}', '\u{f026}', '\u{f027}', '\u{f028}'];
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
        // +1 for adding continuation dots
        pub const MAX_LENGTH: usize = 31;
    }

    pub mod scroll
    {
        pub const INTERVAL: super::Duration = super::Duration::from_secs(60);
    }

    pub mod toggle
    {
        pub const INTERVAL: super::Duration = super::Duration::from_secs(30);
    }
}
