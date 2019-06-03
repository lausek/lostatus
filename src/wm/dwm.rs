#![cfg(feature = "dwm")]
extern crate x11;

use super::*;

unsafe fn set_status(status: String)
{
    use x11::xlib::*;
    let display = XOpenDisplay(std::ptr::null());
    let root_window = XDefaultRootWindow(display);
    XStoreName(display, root_window, status.as_ptr() as *const i8);
}

pub fn output_init() {}

pub fn output_render(_app: &App)
{
    unsafe {
        set_status("status".to_string());
    }
}

pub fn output_error(msg: &str) -> String
{
    msg.to_string()
}

pub fn spawn_system_sender(_sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    None
}
