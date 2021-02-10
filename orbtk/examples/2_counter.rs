use orbtk::prelude::*;

pub fn main() -> Result<(), Error> {
    App::new()
        .window(
            Window::create()
                .title("orbtk: 2_counter example")
                .size(640, 480)
                .centered(true),
        )?
        .start();

    Ok(())
}
