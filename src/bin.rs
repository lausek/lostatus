#![feature(box_patterns)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate i3ipc;
extern crate serde;
extern crate serde_json;

mod config;
#[macro_use]
mod macros;
mod app;
mod i3;
mod scheduler;
mod widget;

use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::app::App;
use crate::config::{widgets, SHELL};
use crate::widget::UpdateEvent;

pub fn shell(cmd: &str) -> std::io::Result<String>
{
    std::process::Command::new(SHELL)
        .args(&["-c", cmd])
        .output()
        .map(|buffer| {
            if !buffer.status.success() {
                panic!(format!(
                    "command `{}` failed: `{:?}`",
                    cmd,
                    buffer.status.code()
                ));
            }
            String::from_utf8(buffer.stdout).unwrap()
        })
}

fn spawn_system_sender(sender: Sender<UpdateEvent>) -> std::thread::JoinHandle<()>
{
    let mut i3 = i3ipc::I3EventListener::connect().expect("i3 not running");

    thread::spawn(move || {
        i3.subscribe(&[i3ipc::Subscription::Window])
            .expect("could not subscribe to i3 events");

        // TODO: does listen block?
        loop {
            for event in i3.listen() {
                match event {
                    Ok(event) => {
                        debug_log!("from system");

                        let sys_event = UpdateEvent::System(Box::new(event));
                        sender.send(sys_event).unwrap();
                    }
                    _ => panic!("system event is err"),
                }
            }
        }
    })
}

fn spawn_user_sender(sender: Sender<UpdateEvent>) -> std::thread::JoinHandle<()>
{
    use crate::i3::I3Input;
    use std::io::BufRead;

    thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            let mut input = line.expect("error on stdin line");
            if input == "[" {
                continue;
            }
            if input.starts_with(',') {
                input = input.split_off(1);
            }

            debug_log!("from i3: {:?}", input);

            match serde_json::from_str::<I3Input>(input.as_ref()) {
                Ok(input) => sender.send(UpdateEvent::User(input)).unwrap(),
                Err(msg) => panic!(format!("invalid json input: {}", msg)),
            }
        }
    })
}

fn setup_panic_hook()
{
    use std::ops::Deref;
    use std::panic;

    panic::set_hook(Box::new(|panic_info| {
        let (filename, line) = panic_info
            .location()
            .map(|loc| (loc.file(), loc.line()))
            .unwrap_or(("<unknown>", 0));

        let cause = panic_info
            .payload()
            .downcast_ref::<String>()
            .map(String::deref);

        let cause = cause.unwrap_or_else(|| {
            panic_info
                .payload()
                .downcast_ref::<&str>()
                .map(|s| *s)
                .unwrap_or("<cause unknown>")
        });

        debug_log!("A panic occurred at {}:{}: {}", filename, line, cause);
    }));
}

fn main() -> Result<(), &'static str>
{
    if cfg!(feature = "debug") {
        setup_panic_hook();
    }

    let (sender, receiver) = channel();

    spawn_system_sender(sender.clone());
    spawn_user_sender(sender.clone());

    App::init(widgets()).run(&receiver)
}
