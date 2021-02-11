use orbtk::prelude::*;

widget!(Counter);

impl Template for Counter {
    fn template(self, _id: Entity, btx: &mut BuildContext) -> Self {
        self.child(TextBlock::new().build(btx))
    }
}

pub fn main() -> Result<(), Error> {
    App::new()
        .window(
            Window::create()
                .title("orbtk: 2_counter example")
                .size(640, 480)
                .centered(true)
                .ui(Counter::new()),
        )?
        .start();

    Ok(())
}
