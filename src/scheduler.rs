use crate::output::I3Block;
use crate::widget::Widget;

pub struct Scheduler<T: Widget>
{
    widgets: Vec<(I3Block, Box<T>)>,
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
                .map(|widget| (I3Block::default(), widget))
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
            for iwidget in &mut self.widgets {
                let ref mut block = iwidget.0;
                let ref mut widget = iwidget.1;

                if let Some(next_update) = widget.update(block) {}

                i3print!("{}", block);
                // TODO: add comma here for every object except last one
            }
            i3print!("],");
        }
    }
}
