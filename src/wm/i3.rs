#![cfg(feature = "i3")]
extern crate i3ipc;

use super::*;

use std::io::Write;

macro_rules! i3print {
    ($msg:expr, $($x:expr),*) => {{ i3print!(format!($msg, $($x),*)); }};
    ($msg:expr) => {{ print!("{}", $msg); }};
}

macro_rules! i3flush {
    () => {{
        std::io::stdout().flush().unwrap();
    }};
}

pub fn output_init()
{
    i3print!("{ \"version\": 1, \"click_events\": true } [");
    i3flush!();
}

pub fn output_render(app: &App)
{
    i3print!("\n[");

    let mut iter = app.widgets.iter();
    let separator = Output::from_text("|");

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

pub fn output_error(msg: &str) -> Output
{
    Output::from_text(msg)
}

pub fn spawn_user_sender(sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    use std::io::BufRead;
    let thread = sender!(user, move || {
        for line in std::io::stdin().lock().lines() {
            let mut input = line.expect("error on stdin line");
            if input == "[" {
                continue;
            }
            if input.starts_with(',') {
                input = input.split_off(1);
            }

            debug_log!("from sender: {:?}", input);

            match serde_json::from_str::<Input>(input.as_ref()) {
                Ok(input) => sender.send(UpdateEvent::User(input)).unwrap(),
                Err(msg) => panic!("invalid json input: {}", msg),
            }
        }
    });
    Some(thread)
}

pub fn spawn_system_sender(sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    let mut i3 = i3ipc::I3EventListener::connect().expect("i3 not running");
    let thread = sender!(system, move || {
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
    });
    Some(thread)
}

impl std::fmt::Display for Output
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
