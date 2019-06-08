#![cfg(feature = "dwm")]

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
            output.push_str(&buffer);
            output
        });
    set_status(output);
}

pub fn output_error(msg: &str) -> String
{
    msg.to_string()
}

fn set_status(status: String)
{
    shell!("xsetroot", "-name", status.as_ref()).unwrap();
}

pub fn spawn_user_sender(_sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    None
}

pub fn spawn_system_sender(_sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    None
}
