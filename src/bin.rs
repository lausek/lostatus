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
mod scheduler;
mod widget;

use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::app::App;
use crate::widget::{UpdateEvent, Widget};

fn main() -> Result<(), &'static str>
{
    // change these
    let widgets: Vec<Box<dyn Widget>> = vec![
        Box::new(widget::Focus::new()),
        Box::new(widget::Battery::new()),
        Box::new(widget::DateTime::new()),
    ];

    let (sender, receiver) = channel();

    spawn_system_sender(&sender);
    spawn_user_sender(&sender);

    App::init(widgets).run(&receiver)
}

fn spawn_system_sender(sender: &Sender<UpdateEvent>) -> std::thread::JoinHandle<()>
{
    let mut i3 = i3ipc::I3EventListener::connect().expect("i3 not running");
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
    })
}

fn spawn_user_sender(sender: &Sender<UpdateEvent>) -> std::thread::JoinHandle<()>
{
    use crate::i3::I3Input;

    let user_sender = sender.clone();

    thread::spawn(move || loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        match serde_json::from_str::<I3Input>(input.as_ref()) {
            Ok(input) => user_sender.send(UpdateEvent::User(input)).unwrap(),
            Err(msg) => panic!(format!("invalid json input: {}", msg)),
        }
    })
}
