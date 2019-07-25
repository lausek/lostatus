use std::fs::File;
use std::sync::Mutex;

pub const LOG_FILE_PATH: &str = "/tmp/lostatus.log";
lazy_static! {
    pub static ref LOG: Mutex<File> =
        Mutex::new(File::create(LOG_FILE_PATH).expect("could not open debug log"));
}

#[macro_export]
macro_rules! shell {
    ($cmd:expr $(, $arg:expr)*) => {{
        // TODO: refactor this
        let cmd = format!("{}", vec![
            format!("{}", $cmd) $(, format!("{}", $arg))*
        ].join(" "));
        match std::process::Command::new(SHELL)
            .args(&["-c", cmd.as_ref()])
            .output()
        {
            Ok(buffer) if buffer.status.success() => Ok(String::from_utf8(buffer.stdout).unwrap()),
            err => Err(format!("{:?}", err)),
        }
    }};
}

#[macro_export]
macro_rules! debug_log {
    ($msg:expr) => {{
        use std::io::Write;
        use crate::app::util::LOG;
        if let Ok(mut lock) = LOG.lock() {
            lock.write_all(b"\n").unwrap();
            lock.write_all($msg.as_bytes()).unwrap();
        }
    }};
    ($msg:expr, $($x:expr),*) => {{
        debug_log!(format!($msg, $($x),*));
    }};
}

pub fn get_percentage_char(percentage: f64, from: &[char]) -> char
{
    let idx = (percentage / 101.0 * from.len() as f64).floor() as usize;
    from[idx]
}
