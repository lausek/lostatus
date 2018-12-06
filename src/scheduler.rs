use crate::output::I3Block;
use crate::widget::Widget;

pub struct Scheduler<T: Widget>
{
    widgets: Vec<(String, Box<T>)>,
}

impl<T> Scheduler<T>
where
    T: Widget,
{
    pub fn from_widgets(widgets: Vec<Box<T>>) -> Self
    {
        Self {
            widgets: widgets
                .into_iter()
                .map(|mut widget| {
                    if let Some((output, next_update)) = widget.update() {
                        return (format!("{}", output), widget);
                    }
                    (String::from("error on block"), widget)
                })
                .collect::<Vec<_>>(),
        }
    }

    pub fn push(&mut self)
    {
        // push next gadget into queue
    }

    pub fn run(&mut self)
    {
        i3print!("{ \"version\": 1 }");

        i3print!("[");

        loop {
            i3print!("[");
            for (ref block, _) in &mut self.widgets {
                i3print!(block);
                // TODO: add comma here for every object except last one
            }
            i3print!("],");
        }
    }
}
