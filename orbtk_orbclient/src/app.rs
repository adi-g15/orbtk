use crate::{Error, Window, WindowBuilder};

/// Manges the initial setup of the application, its startup and the application loop.
pub struct App {
    windows: Vec<Window>,
}

impl App {
    /// Creates a new app, that can be configured using the builder pattern,.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use orbtk_orbclient::{App, Window};
    /// // use orbtk::orbclient::{App, Window};
    ///
    /// App::new().window(
    ///    Window::create().position(0, 0).size(640, 480).title("Window")
    /// ).unwrap().start();
    /// ```
    pub fn new() -> App {
        App { windows: vec![] }
    }

    /// Builder method that is used add a new window to the app.
    pub fn window(mut self, builder: WindowBuilder) -> Result<Self, Error> {
        self.windows.push(builder.build()?);
        Ok(self)
    }

    // Starts and runs the application (not wasm32 target)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start(mut self) {
        // todo: handle animation loop.
        loop {
            if !self.run() {
                break;
            }
        }
    }

    // Starts and runs the application (wasm32 target)
    #[cfg(target_arch = "wasm32")]
    pub fn start(mut self) {
        animation_loop(move || self.run());
    }

    // runs the loop
    fn run(&mut self) -> bool {
        for i in 0..self.windows.len() {
            // removes the window from the list if run returns false
            if !self.windows[i].drain_events() {
                self.windows.remove(i);
            }
        }

        // if no window is left, close the application.
        if self.windows.is_empty() {
            return false;
        }

        return true;
    }
}
