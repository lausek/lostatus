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

    pub fn run(&mut self)
    {
        i3print!("{ \"version\": 1 }");

        i3print!("[");

        loop {
            i3print!("[");
            for iwidget in &mut self.widgets {
                let ref mut block = iwidget.0;
                let ref mut widget = iwidget.1;

                let next_update = widget.update(block);

                i3print!("{},", block);
            }
            i3print!("],");
        }
    }
}
