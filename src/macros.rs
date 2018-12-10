macro_rules! i3print {
    ($msg:expr) => {{
        println!("{}", $msg);
    }};
    ($msg:expr, $($x:expr),*) => {{
        i3print!(format!($msg, $($x),*))
    }};
}

macro_rules! i3error {
    ($msg:expr) => {
        // TODO: add color
        format!("{{\"full_text\": \"{}\"}}", $msg)
    };
}
