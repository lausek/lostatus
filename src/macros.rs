use std::fs::File;
use std::sync::Mutex;

lazy_static! {
    pub static ref LOG: Mutex<File> =
        Mutex::new(File::create("/tmp/lostatus.log").expect("could not open debug log"));
}

macro_rules! debug_log {
    ($msg:expr) => {if cfg!(feature = "debug") {
        use std::io::Write;
        use crate::macros::LOG;
        if let Ok(mut lock) = LOG.lock() {
            lock.write(b"\n").unwrap();
            lock.write($msg.as_bytes()).unwrap();
        }
    }};
    ($msg:expr, $($x:expr),*) => {if cfg!(feature = "debug") {
        debug_log!(format!($msg, $($x),*));
    }};
}

macro_rules! i3print {
    ($msg:expr, $($x:expr),*) => {{ i3print!(format!($msg, $($x),*)); }};
    ($msg:expr) => {{ print!("{}", $msg); }};
}

macro_rules! i3flush {
    () => {{
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }};
}

macro_rules! i3error {
    ($msg:expr) => {
        // TODO: add color
        format!("{{\"full_text\": \"{}\"}}", $msg)
    };
}
