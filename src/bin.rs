#[macro_use]
extern crate serde_derive;

extern crate i3ipc;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
mod output;
mod scheduler;
mod widget;

use std::thread;

use crate::scheduler::Scheduler;

fn main()
{
    // change these
    let widgets = vec![
        Box::new(widget::Battery::new()),
        Box::new(widget::DateTime::new()),
    ];

    let mut i3 = i3ipc::I3EventListener::connect().expect("i3 not running");
    let mut scheduler = Scheduler::from_widgets(widgets);

    thread::spawn(move || {
        i3.subscribe(&[i3ipc::Subscription::Window])
            .expect("could not subscribe to i3 events");

        // TODO: does listen block?
        loop {
            for event in i3.listen() {
                println!("{:?}", event);
            }
        }
    });

    scheduler.run();
}
