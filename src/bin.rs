#![feature(box_patterns)]
#![feature(bind_by_move_pattern_guards)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "i3")]
extern crate i3ipc;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod app;
mod config;
mod widget;
mod wm;

use crate::app::*;
use crate::config::*;
use crate::widget::*;
use crate::wm::*;

use std::sync::mpsc::{channel, Sender};
use std::thread;

fn main() -> Result<(), &'static str>
{
    if cfg!(feature = "debug") {
        setup_panic_hook();
    }

    let (sender, receiver) = channel();

    spawn_user_sender(sender.clone());
    spawn_system_sender(sender.clone());

    App::init(widgets()).run(&receiver)
}

fn spawn_user_sender(sender: Sender<UpdateEvent>) -> std::thread::JoinHandle<()>
{
    use std::io::BufRead;

    // TODO: introduce generic input interface for frontends
    thread::spawn(move || {
        for line in std::io::stdin().lock().lines() {
            let mut input = line.expect("error on stdin line");
            if input == "[" {
                continue;
            }
            if input.starts_with(',') {
                input = input.split_off(1);
            }

            debug_log!("from sender: {:?}", input);

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
