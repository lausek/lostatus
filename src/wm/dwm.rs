#![cfg(feature = "dwm")]
extern crate x11;

use super::*;

use x11::xlib::*;

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
    unsafe {
        let (display, root) = x11comps();
        XStoreName(display, root, status.as_ptr() as *const i8);
        // TODO: check if needed
        XCloseDisplay(display);
    }
}

pub fn spawn_system_sender(_sender: Sender<UpdateEvent>) -> Option<std::thread::JoinHandle<()>>
{
    let thread = system_sender!(move || unsafe {
        let (display, root) = x11comps();
        let mut event = XEvent { type_: 0 };
        XSelectInput(display, root, ButtonPressMask);
        loop {
            XNextEvent(display, &mut event as *mut XEvent);
            match event {
                XEvent { button } => {
                    println!("button pressed");
                }
                _ => {}
            }
        }
    });
    Some(thread)
}

unsafe fn x11comps() -> (*mut Display, Window)
{
    match XOpenDisplay(std::ptr::null()) {
        n if !n.is_null() => Some((n, XDefaultRootWindow(n))),
        _ => None,
    }
    .expect("display not open.")
}
