pub mod i3;
#[macro_use]
pub mod macros;
pub mod scheduler;
pub mod util;

pub use self::i3::*;

use self::scheduler::Scheduler;
use crate::config::*;
use crate::widget::{UpdateEvent, Widget};

use std::str::FromStr;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

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

        for (id, mut widget) in &mut app.widgets.iter_mut().enumerate() {
            update(&mut app.scheduler, (id, &mut widget), &UpdateEvent::Time);
            if widget.0.is_empty() {
                // TODO: this needs an instance id or it will crash on click
                //       e.g. battery in qemu does not exist -> no instance id set
                widget.0 = format!("{}", I3Output::default());
            }
        }

        app
    }

    pub fn render(&self)
    {
        i3print!("\n[");

        let mut iter = self.widgets.iter();
        let separator = I3Output::from_text("|");

        if let Some((ref first, _)) = iter.next() {
            i3print!(first);
        }

        for (ref block, _) in iter {
            i3print!(",{}", separator);
            i3print!(",{}", block);
        }

        i3print!("],");
        i3flush!();
    }

    pub fn run(&mut self, receiver: &Receiver<UpdateEvent>) -> Result<(), &'static str>
    {
        use self::UpdateEvent::*;

        i3print!("{ \"version\": 1, \"click_events\": true } [");
        i3flush!();

        self.render();

        // TODO: add `quit` event
        loop {
            let event = if let Some(timeout) = self.timeout {
                receiver.recv_timeout(timeout)
            } else {
                receiver.recv_timeout(INTERVAL)
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
                    let id = usize::from_str(input.instance.as_ref())
                        .expect("invalid instance id supplied");
                    if let Some(widget) = self.widgets.get_mut(id) {
                        update(&mut self.scheduler, (id, widget), &UpdateEvent::User(input));
                    }
                }
                Ok(Time) | Err(RecvTimeoutError::Timeout) => debug_log!("time was sent"),
                Err(RecvTimeoutError::Disconnected) => panic!("sending channel got killed"),
            }

            // test if events occurred while working on another one
            for id in self.scheduler.get_due_ids() {
                let mut widget = self.widgets.get_mut(id).expect("inconsistent ids");
                update(&mut self.scheduler, (id, &mut widget), &UpdateEvent::Time);
            }
            // set timed update for the next cycle
            self.timeout = self.scheduler.next_update();

            self.render();
        }
    }
}

fn update<T>(scheduler: &mut Scheduler, widget: (usize, &mut (String, Box<T>)), evt: &UpdateEvent)
where
    T: Widget + ?Sized,
{
    use crate::config::COLOR_SCHEME;

    let (id, (cache, widget)) = widget;
    // if `update` returns None: nothing to update
    if let Some((i3output, next_update)) = widget.update(evt) {
        *cache = match i3output {
            Ok(mut i3output) => {
                i3output.instance = Some(format!("{}", id));

                if i3output.color.is_none() {
                    i3output.color = Some(COLOR_SCHEME.basic.foreground.to_string());
                }

                if i3output.background.is_none() {
                    i3output.background = Some(COLOR_SCHEME.basic.background.to_string());
                }

                format!("{}", i3output)
            }
            Err(msg) => i3error!(msg),
        };

        scheduler.schedule(id, next_update);
    }
}