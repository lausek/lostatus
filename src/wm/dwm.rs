use super::*;

#[cfg(feature = "dwm")]
pub fn output_init() {}

#[cfg(feature = "dwm")]
pub fn output_render(_app: &App) {}

#[cfg(feature = "dwm")]
pub fn output_error(_msg: &str) -> String
{
    unimplemented!()
}

#[cfg(feature = "dwm")]
pub fn spawn_system_sender(sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    None
}
