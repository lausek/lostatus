#![feature(box_patterns)]
#![feature(bind_by_move_pattern_guards)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

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

fn main() -> Result<(), &'static str>
{
    if cfg!(debug_assertions) {
        setup_panic_hook();
    }

    let (sender, receiver) = channel();

    let _handles = spawn_senders(sender.clone());

    App::init(widgets()).run(&receiver)
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
