use super::*;

pub mod output;
pub mod scheduler;
#[macro_use]
pub mod util;

pub use self::output::*;
pub use self::scheduler::*;
pub use self::util::*;

use std::str::FromStr;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

pub struct App
{
    pub(crate) widgets: Vec<(Output, Box<dyn Widget>)>,
    pub(crate) timeout: Option<Duration>,
    pub(crate) scheduler: Scheduler,
}

impl App
{
    pub fn init(widgets: Vec<Box<dyn Widget>>) -> Self
    {
        let mut app = Self {
            timeout: None,
            scheduler: Scheduler::new(widgets.len()),
            widgets: widgets
                .into_iter()
                .map(|w| (Output::new(), w))
                .collect::<Vec<_>>(),
        };

        for (id, mut widget) in &mut app.widgets.iter_mut().enumerate() {
            update(&mut app.scheduler, (id, &mut widget), &UpdateEvent::Time);
        }

        app
    }

    pub fn render(&self)
    {
        output_render(&self);
    }

    pub fn run(&mut self, receiver: &Receiver<UpdateEvent>) -> Result<(), &'static str>
    {
        use self::UpdateEvent::*;

        output_init();

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

fn update<T>(scheduler: &mut Scheduler, widget: (usize, &mut (Output, Box<T>)), evt: &UpdateEvent)
where
    T: Widget + ?Sized,
{
    let (id, (cache, widget)) = widget;
    // if `update` returns None: nothing to update
    if let Some((output, next_update)) = widget.update(evt) {
        *cache = match output {
            Ok(mut output) => {
                output.instance = Some(format!("{}", id));

                if output.color.is_none() {
                    output.color = Some(COLOR_SCHEME.basic.foreground.to_string());
                }

                if output.background.is_none() {
                    output.background = Some(COLOR_SCHEME.basic.background.to_string());
                }

                output
            }
            Err(msg) => output_error(msg),
        };

        scheduler.schedule(id, next_update);
    }
}
