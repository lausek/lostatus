use std::fs::File;
use std::sync::Mutex;

lazy_static! {
    pub static ref LOG: Mutex<File> =
        Mutex::new(File::create("/tmp/lostatus.log").expect("could not open debug log"));
}

macro_rules! debug_log {
    ($msg:expr) => {if cfg!(feature = "debug") {
        use std::io::Write;
        use crate::app::macros::LOG;
        if let Ok(mut lock) = LOG.lock() {
            lock.write_all(b"\n").unwrap();
            lock.write_all($msg.as_bytes()).unwrap();
        }
    }};
    ($msg:expr, $($x:expr),*) => {if cfg!(feature = "debug") {
        debug_log!(format!($msg, $($x),*));
    }};
}
