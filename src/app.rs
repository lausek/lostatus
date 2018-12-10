use std::sync::mpsc::Receiver;

use crate::widget::{UpdateEvent, Widget};

pub struct App<T: Widget>
{
    widgets: Vec<(String, Box<T>)>,
}

impl<T> App<T>
where
    T: Widget,
{
    pub fn init(widgets: Vec<Box<T>>) -> Self
    {
        Self {
            widgets: widgets
                .into_iter()
                .map(|mut widget| {
                    if let Some((Ok(output), next_update)) = widget.update(&UpdateEvent::Time) {
                        return (format!("{}", output), widget);
                    }
                    (String::from("error on block"), widget)
                })
                .collect::<Vec<_>>(),
        }
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
                    self.widgets
                        .iter_mut()
                        .filter(|w| w.1.needs_system())
                        .for_each(move |w| {
                            // TODO: pass sys_event here
                            w.1.update(&sys_event);
                        });
                }
                User(input) => {}
                Time => {}
            }
            self.render();
        }

        Ok(())
    }
}
