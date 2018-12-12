use std::ops::Add;
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
            Some(next.unwrap().duration_since(SystemTime::now()).unwrap())
        } else {
            None
        }
    }

    pub fn get_due_ids(&self) -> Vec<usize>
    {
        let now = SystemTime::now();
        let mut ids = vec![];
        let mut iter = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.is_some());
        for (id, slot) in iter {
            if slot.unwrap() <= now {
                ids.push(id);
            }
        }
        ids
    }
}
