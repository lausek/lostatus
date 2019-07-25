#![cfg(not(any(feature = "i3", feature = "dwm")))]

use super::*;

pub fn output_init() {}

pub fn output_render(app: &App)
{
    let output = app
        .widgets
        .iter()
        .fold(String::new(), |mut output, (buffer, _)| {
            if !output.is_empty() {
                output.push_str("|");
            }
            output.push_str(&buffer.full_text);
            output
        });
    println!("{}", output);
}

pub fn output_error(msg: &str) -> Output
{
    Output::from_text(msg)
}

pub fn spawn_user_sender(_sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    None
}

pub fn spawn_system_sender(_sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    None
}
