#[cfg(feature = "dwm")]
extern crate x11;

use super::*;

unsafe fn set_status(mut status: String)
{
    use x11::xlib::*;
    let display = XOpenDisplay(std::ptr::null());
    let root_window = XDefaultRootWindow(display);
    XStoreName(display, root_window, status.as_ptr() as *const i8);
}

#[cfg(feature = "dwm")]
pub fn output_init() {}

#[cfg(feature = "dwm")]
pub fn output_render(_app: &App)
{
    unsafe {
        set_status("status".to_string());
    }
}

#[cfg(feature = "dwm")]
pub fn output_error(msg: &str) -> String
{
    msg.to_string()
}

#[cfg(feature = "dwm")]
pub fn spawn_system_sender(_sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    None
}
