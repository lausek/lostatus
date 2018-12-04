macro_rules! i3print {
    ($msg:expr) => {{
        println!("{}", $msg);
    }};
    ($msg:expr, $($x:expr),*) => {{
        i3print!(format!($msg, $($x),*))
    }};
}
