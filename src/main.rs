extern crate i3ipc;

#[macro_use]
mod macros;
mod output;
mod scheduler;
mod widget;

use std::thread;

use crate::scheduler::Scheduler;

fn main()
{
    let widgets = vec![Box::new(widget::Time::new())];

    let mut i3 = i3ipc::I3EventListener::connect().expect("i3 not running");
    let mut scheduler = Scheduler::from_widgets(widgets);

    thread::spawn(move || {
        i3.subscribe(&[i3ipc::Subscription::Window])
            .expect("could not subscribe to i3 events");

        // TODO: is listen blocking?
        loop {
            for event in i3.listen() {
                println!("{:?}", event);
            }
        }
    });

    scheduler.run();
}
