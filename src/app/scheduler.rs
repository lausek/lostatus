use super::*;

use std::time::{Duration, SystemTime};

pub struct Scheduler
{
    slots: Vec<Option<SystemTime>>,
}

impl Scheduler
{
    pub fn new(size: usize) -> Self
    {
        Self {
            slots: vec![None; size],
        }
    }

    pub fn schedule(&mut self, id: usize, after: Option<Duration>)
    {
        if let Some(slot) = self.slots.get_mut(id) {
            *slot = if let Some(duration) = after {
                Some(SystemTime::now() + duration)
            } else {
                None
            };
        }
    }

    pub fn next_update(&self) -> Option<Duration>
    {
        let mut iter = self.slots.iter().filter(|slot| slot.is_some());
        if let Some(first) = iter.next() {
            let mut next = first;
            for slot in iter {
                if slot < next {
                    next = slot;
                }
            }
            match next.unwrap().duration_since(SystemTime::now()) {
                Ok(duration) => Some(duration),
                // TODO: update is overdue. this should do something
                Err(_) => {
                    debug_log!("scheduler queue: {:?}", next);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn get_due_ids(&self) -> Vec<usize>
    {
        let now = SystemTime::now();
        self.slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.is_some() && slot.unwrap() <= now)
            .map(|(id, _)| id)
            .collect()
    }
}
