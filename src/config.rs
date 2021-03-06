#![allow(dead_code)]

use super::*;

use std::str::FromStr;
use std::time::Duration;

pub const SHELL: &str = "fish";
pub const INTERVAL: Duration = Duration::from_secs(4);
pub const COLOR_SCHEME: ColorScheme = ColorScheme {
    basic: Color {
        foreground: "#cfd8dc",
        background: "#222d32",
    },
    good: Color {
        foreground: "#1d1f21",
        background: "#99b938",
    },
    degraded: Color {
        foreground: "#1d1f21",
        background: "#fe7e29",
    },
    bad: Color {
        foreground: "#1d1f21",
        background: "#ff5252",
    },
};

pub struct Color
{
    pub foreground: &'static str,
    pub background: &'static str,
}

pub struct ColorScheme
{
    pub basic: Color,
    pub good: Color,
    pub degraded: Color,
    pub bad: Color,
}

// TODO: make declaration nicer with procedural macro?
pub fn widgets() -> Vec<Box<dyn Widget>>
{
    use Action::*;
    let volume = Scroll::new()
        .command(ScrollUp, "~/.config/scripts/volume up")
        .command(ScrollDown, "~/.config/scripts/volume down")
        .command(Status, "~/.config/scripts/volume")
        .formatter(|state| {
            state
                .cmd_status
                .as_ref()
                .map_or(Err("no status cmd"), |cmd| match shell!(cmd) {
                    Ok(output) => {
                        let status = output.lines().next().unwrap().to_string();
                        let icon = match f64::from_str(status.as_ref()) {
                            Ok(volume) => get_percentage_char(volume, &chars::VOLUME),
                            _ => panic!("could not interpret status command as number"),
                        };
                        Ok(format!("{} {}", icon, status))
                    }
                    _ => Err("status command failed"),
                })
        });

    // TODO: this block is way too slow
    let _brightness = Scroll::new()
        .command(ScrollUp, "~/.config/scripts/brightness-lostatus +5")
        .command(ScrollDown, "~/.config/scripts/brightness-lostatus -5")
        .command(Status, "~/.config/scripts/brightness-lostatus")
        .formatter(|state| {
            state
                .cmd_status
                .as_ref()
                .map_or(Err("no status cmd"), |cmd| match shell!(cmd) {
                    Ok(output) => {
                        let status = output.lines().next().unwrap().to_string();
                        let icon = match f64::from_str(status.as_ref()) {
                            Ok(volume) => get_percentage_char(volume, &chars::VOLUME),
                            _ => panic!("could not interpret status command as number"),
                        };
                        Ok(format!("{} {}", icon, status))
                    }
                    _ => Err("status command failed"),
                })
        });

    let speakers = Toggle::new()
        .command("~/.config/scripts/headset-switch-toggle")
        .formatter(|_| Ok("Speakers".to_string()));

    vec![
        Box::new(Focus::new()),
        //Box::new(brightness),
        Box::new(volume),
        Box::new(speakers),
        Box::new(Battery::new()),
        Box::new(DateTime::new()),
    ]
}

pub mod chars
{
    pub const BARS: &[char] = &[' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    pub const BRIGHTNESS: char = '☼';
    pub const CONTINUE: char = '\u{2026}';
    pub const IO: &[char] = &['0', '1'];
    pub const MUSIC: char = '\u{e405}';
    pub const VOLUME: &[char] = &['\u{f00d}', '\u{f026}', '\u{f027}', '\u{f028}'];
}

pub const BATTERY_INTERVAL: Duration = Duration::from_secs(60);
pub const BATTERY_FILE_PATH: &str = "/sys/class/power_supply/BAT0/capacity";

pub const DATETIME_INTERVAL: Duration = Duration::from_secs(30);
pub const DATETIME_FORMAT: &str = "date +\"%H:%M / %d.%m.%Y\"";

pub const FOCUS_INTERVAL: Duration = Duration::from_secs(60);
// +1 for adding continuation dots
pub const FOCUS_MAX_LENGTH: usize = 31;

pub const SCROLL_INTERVAL: Duration = Duration::from_secs(60);
pub const TOGGLE_INTERVAL: Duration = Duration::from_secs(30);
