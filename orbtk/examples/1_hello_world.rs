use orbtk::*;

pub fn main() -> Result<(), Error> {
    App::new()
        .window(
            Window::create()
                .title("orbtk: 1_hello_world example")
                .size(640, 480)
                .centered(true),
        )?
        .start();

    Ok(())
}
