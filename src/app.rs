use std::iter::Enumerate;
use std::slice::IterMut;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

use crate::scheduler::Scheduler;
use crate::widget::{UpdateEvent, Widget};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(5);

pub struct App<T: Widget + ?Sized>
{
    widgets: Vec<(String, Box<T>)>,
    timeout: Option<Duration>,
    scheduler: Scheduler,
}

impl<T> App<T>
where
    T: Widget + ?Sized,
{
    pub fn init(widgets: Vec<Box<T>>) -> Self
    {
        let mut app = Self {
            timeout: None,
            scheduler: Scheduler::new(widgets.len()),
            widgets: widgets
                .into_iter()
                .map(|w| (String::new(), w))
                .collect::<Vec<_>>(),
        };

        for mut widget in &mut app.widgets.iter_mut().enumerate() {
            update(&mut app.scheduler, widget, &UpdateEvent::Time);
        }

        app
    }

    pub fn render(&self)
    {
        i3print!("[");

        let mut iter = self.widgets.iter();

        if let Some((ref first, _)) = iter.next() {
            i3print!(first);
        }

        for (ref block, _) in iter {
            i3print!(",{}", block);
        }

        i3print!("],");
    }

    pub fn run(&mut self, receiver: &Receiver<UpdateEvent>) -> Result<(), &'static str>
    {
        use self::UpdateEvent::*;

        i3print!("{ \"version\": 1 }");

        i3print!("[");

        self.render();

        loop {
            let event = if let Some(timeout) = self.timeout {
                receiver.recv_timeout(timeout)
            } else {
                receiver.recv_timeout(DEFAULT_TIMEOUT)
            };

            match event {
                Ok(sys_event @ System(_)) => {
                    let mut iter = self
                        .widgets
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, (_, w))| w.needs_system());

                    while let Some(widget) = iter.next() {
                        update(&mut self.scheduler, widget, &sys_event);
                    }
                    self.timeout = self.scheduler.next_update();
                }
                Ok(User(input)) => {
                    let id = usize::from_str(input.instance.as_ref()).unwrap();
                    if let Some(widget) = self.widgets.get_mut(id) {
                        update(&mut self.scheduler, (id, widget), &UpdateEvent::User(input));
                        self.timeout = self.scheduler.next_update();
                    }
                }
                Ok(Time) | Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => panic!("sending channel got killed"),
            }

            for id in self.scheduler.get_due_ids() {
                let widget = self.widgets.get(id);
            }

            self.render();
        }

        Ok(())
    }
}

fn update<T>(scheduler: &mut Scheduler, widget: (usize, &mut (String, Box<T>)), evt: &UpdateEvent)
where
    T: Widget + ?Sized,
{
    let (id, (cache, widget)) = widget;
    // if `update` returns None: nothing to update
    if let Some((i3output, next_update)) = widget.update(evt) {
        *cache = match i3output {
            Ok(mut i3output) => {
                i3output.instance = Some(format!("{}", id));

                if i3output.full_text.is_none() {
                    i3output.full_text = Some(String::new());
                }

                format!("{}", i3output)
            }
            Err(msg) => i3error!(msg),
        };

        scheduler.schedule(id, next_update);
    }
}
