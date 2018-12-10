#![feature(box_patterns)]

#[macro_use]
extern crate serde_derive;

extern crate i3ipc;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
mod app;
mod i3;
mod widget;

use std::sync::mpsc::channel;
use std::thread;

use crate::app::App;
use crate::widget::{UpdateEvent, Widget};

fn main()
{
    // change these
    let widgets = vec![
        Box::new(widget::Battery::new()),
        //Box::new(widget::DateTime::new()),
    ];

    let mut i3 = i3ipc::I3EventListener::connect().expect("i3 not running");
    let (sender, receiver) = channel();

    let system_sender = sender.clone();
    thread::spawn(move || {
        i3.subscribe(&[i3ipc::Subscription::Window])
            .expect("could not subscribe to i3 events");

        // TODO: does listen block?
        loop {
            for event in i3.listen() {
                match event {
                    Ok(event) => {
                        let sys_event = UpdateEvent::System(Box::new(event));
                        system_sender.send(sys_event).unwrap();
                    }
                    _ => panic!("system event is err"),
                }
            }
        }
    });

    let user_sender = sender.clone();
    thread::spawn(move || { /* user events */ });

    let time_sender = sender.clone();
    thread::spawn(move || { /* timed events */ });

    App::init(widgets).run(&receiver);
}
