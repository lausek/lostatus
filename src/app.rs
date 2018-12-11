use std::sync::mpsc::Receiver;

use crate::widget::{UpdateEvent, Widget};

pub struct App<T: Widget + ?Sized>
{
    widgets: Vec<(String, Box<T>)>,
}

impl<T> App<T>
where
    T: Widget + ?Sized,
{
    pub fn init(widgets: Vec<Box<T>>) -> Self
    {
        let mut app = Self {
            widgets: widgets
                .into_iter()
                .map(|w| (String::new(), w))
                .collect::<Vec<_>>(),
        };

        app.widgets
            .iter_mut()
            .for_each(|widget| update(widget, &UpdateEvent::Time));

        app
    }

    pub fn render(&self)
    {
        i3print!("[");

        let mut iter = self.widgets.iter();

        if let Some((ref first, _)) = iter.next() {
            i3print!(first);
        }

        for (ref block, _) in iter {
            i3print!(",{}", block);
        }

        i3print!("],");
    }

    pub fn run(&mut self, receiver: &Receiver<UpdateEvent>) -> Result<(), &'static str>
    {
        use self::UpdateEvent::*;

        i3print!("{ \"version\": 1 }");

        i3print!("[");

        while let Ok(event) = receiver.recv() {
            match event {
                sys_event @ System(_) => {
                    let mut iter = self.widgets.iter_mut().filter(|w| w.1.needs_system());

                    while let Some(widget) = iter.next() {
                        update(widget, &sys_event);
                    }
                }
                User(input) => println!("json made me {:?}", input),
                Time => {}
            }
            self.render();
        }

        Ok(())
    }
}

fn update<T>(widget: &mut (String, Box<T>), evt: &UpdateEvent)
where
    T: Widget + ?Sized,
{
    // if `update` returns None: nothing to update
    if let Some((i3output, next_update)) = widget.1.update(evt) {
        widget.0 = match i3output {
            Ok(i3output) => format!("{}", i3output),
            Err(msg) => i3error!(msg),
        };

        if let Some(next_update) = next_update {
            // TODO: set next update
        }
    }
}
