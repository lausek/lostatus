use super::*;

use std::str::FromStr;

pub struct Battery {}

impl Battery
{
    pub fn new() -> Self
    {
        Self {}
    }
}

impl Widget for Battery
{
    fn update(&mut self, _evt: &UpdateEvent) -> Option<(BlockResult, Option<Duration>)>
    {
        // TODO: only do timed updates here
        match std::fs::read_to_string(BATTERY_FILE_PATH) {
            Ok(content) => Some((
                if let Some(content) = content.lines().next() {
                    match f64::from_str(content) {
                        Ok(capacity) => {
                            let symbol = get_percentage_char(capacity, &chars::BARS);
                            Ok(Output::from_text(format!("{}", symbol)))
                        }
                        _ => Err("invalid content"),
                    }
                } else {
                    Err("invalid capacity")
                },
                Some(BATTERY_INTERVAL),
            )),
            Err(_) => Some((Err("no battery"), None)),
        }
    }
}
