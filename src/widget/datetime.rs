use super::*;

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
        match *evt {
            UpdateEvent::Time => {
                let result = match shell!(DATETIME_FORMAT) {
                    Ok(date) => {
                        let normalized = date.lines().next().unwrap();
                        Ok(Output::from_text(normalized))
                    }
                    _ => Err("date failed"),
                };
                Some((result, Some(DATETIME_INTERVAL)))
            }
            _ => None,
        }
    }
}
