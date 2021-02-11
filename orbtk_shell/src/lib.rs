//! Immediate mode user interface (ui) shell.

use dces::prelude::*;

use orbtk_render::RenderContext2D;

use orbtk_api::{
    application::ContextProvider,
    localization::*,
    services::*,
    theming::Theme,
    tree::*,
    widget_base::{BuildContext, Widget},
};
use orbtk_utils::*;

mod systems;
use self::systems::*;

/// Represents an immediate mode user interface (ui) shell.
///
/// It contains the Entity Component System (ECS), that organizes all the widgets of
/// the ui.
///
/// The shell also give access to a frame buffer, that represents the result of the rendered
/// ui. The frame buffer e.g. can be drawn to a window or an image.
///
/// It also handles events from outside that can be injected e.g. from a window.
pub struct Shell {
    world: World<Tree>,
    frame_buffer: Vec<u8>,
    mouse_position: Point,
    // 0 => left, 1 => middle, 2 => right
    mouse_button_states: (bool, bool, bool),
    size: (u32, u32),

    dummy_pos: (i32, i32),
}

impl Shell {
    /// Creates a new shell builder that is used to configure the shell.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use morph_ui_shell::Shell;
    /// // use morph_ui::shell::Shell;
    ///
    /// let shell = Shell::create().ui(|ctx| {}).build();
    /// ```
    pub fn create() -> ShellBuilder {
        ShellBuilder::new(String::default())
    }

    /// Creates a new shell builder with an app name that is used to configure the shell.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use morph_ui_shell::Shell;
    /// // use morph_ui::shell::Shell;
    ///
    /// let shell = Shell::create_from_name("my app").ui(|ctx| {}).build();
    /// ```
    pub fn create_from_name(app_name: String) -> ShellBuilder {
        ShellBuilder::new(app_name)
    }

    /// Resizes the shell (frame buffer) to the given size.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = (width, height);
        self.frame_buffer = vec![0; width as usize * height as usize * 4];

        if let Some(rtx) = self.world.try_resource_mut::<RenderContext2D>() {
            rtx.resize(width as f64, height as f64);
        }
        // todo: render
    }

    /// Injects the current position of the mouse pointer.
    pub fn mouse(&mut self, x: f64, y: f64) {
        self.mouse_position = Point::new(x, y);
        println!(
            "mouse not yet implemented, mouse position: {:?}",
            self.mouse_position
        );
    }

    /// Injects the current state of mouse buttons. If `true` the corresponding button
    /// is pressed.
    pub fn mouse_button(&mut self, left: bool, middle: bool, right: bool) {
        // let (mouse_button, pressed) = {
        //     if left != self.mouse_button_states.0 {
        //         (MouseButton::Left, left)
        //     } else if middle != self.mouse_button_states.1 {
        //         (MouseButton::Middle, middle)
        //     } else {
        //         (MouseButton::Right, right)
        //     }
        // };

        // let mouse_event = MouseEvent::new(self.mouse_position, mouse_button, pressed);

        // println!(
        //     "mouse button not yet implemented, mouse event: {:?}",
        //     mouse_event
        // );
        // self.mouse_button_states = (left, middle, right);
    }

    /// Injects the delta x and y of a scroll event.
    pub fn scroll(&mut self, delta_x: f64, delta_y: f64) {
        println!("Scroll delta (x: {}, y: {})", delta_x, delta_y);
    }

    /// Injects the state of a keyboard key if it is changed between `up` and `down`.
    pub fn key(&mut self, scan_code: u8, character: char, pressed: bool) {
        // let key_event = KeyEvent::new(scan_code, character, pressed);

        // if key_event.key() == Key::ArrowRight && key_event.pressed() {
        //     self.dummy_pos.0 += 10;
        // }

        // if key_event.key() == Key::ArrowLeft && key_event.pressed() {
        //     self.dummy_pos.0 -= 10;
        // }

        // if key_event.key() == Key::ArrowDown && key_event.pressed() {
        //     self.dummy_pos.1 += 10;
        // }

        // if key_event.key() == Key::ArrowUp && key_event.pressed() {
        //     self.dummy_pos.1 -= 10;
        // }

        // println!("key not yet implemented, key event: {:?}", key_event);
    }

    /// Injects the text of a text input event.
    pub fn text_input(&mut self, _text: String) {}

    /// Quits the shell.
    pub fn quit(&mut self) {}

    /// Sets the shell as activated (focusedt).
    pub fn active(&mut self, active: bool) {}

    /// Injects the current position of the container e.g. a Window after its position is changed.
    pub fn moved(&mut self, x: f64, y: f64) {}

    pub fn update_and_draw(&mut self) {
        // // todo handle error.
        // let mut rctx = RenderContext::new(self.size.0, self.size.1).unwrap();
        // rctx.set_fill_brush("blue");
        // rctx.fill_rect(self.dummy_pos.0 as f64, self.dummy_pos.1 as f64, 200., 200.);

        if let Some(rtx) = self.world.try_resource_mut::<RenderContext2D>() {
            self.frame_buffer.copy_from_slice(rtx.data_u8_mut());
        }

        // todo: iterator over all entities by name, build query with render component parts like text and text style
    }

    /// Returns a reference to the frame buffer.
    pub fn frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    /// Returns a mutable reference to the frame buffer.
    pub fn frame_buffer_mut(&mut self) -> &mut [u8] {
        &mut self.frame_buffer
    }
}

/// Shell builder is used to configure a shell and create it.
pub struct ShellBuilder {
    world: World<Tree>,
    size: (u32, u32),
    app_name: String,
}

impl ShellBuilder {
    /// Creates a new shell builder.
    fn new(app_name: String) -> Self {
        ShellBuilder {
            world: World::from_entity_store(Tree::new()),
            size: (0, 0),
            app_name,
        }
    }

    /// Builder method that is used to specify the shell size with width and height.
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    /// Builder method that is used to specify the theme of the ui.
    pub fn theme(mut self, theme: impl Into<Theme>) -> Self {
        self.world.insert_resource(theme.into());
        self
    }

    /// Builder method that is used to specify the localization service that is used to localize the ui.
    pub fn localization(mut self, localization: impl Into<RonLocalization>) -> Self {
        self.world.insert_resource(localization.into());
        self
    }

    /// Builder method to define a widget as root of the shells ui.
    ///
    /// An ui can only be set once. If the method is multiple called, the last set ui will be used.
    pub fn ui(mut self, ui: impl Widget) -> Self {
        // let (sender, receiver) = mpsc::channel();

        // let context_provider =
        //     ContextProvider::new(sender, request_sender, app_name.clone(), localization);

        // if app_name.is_empty() {
        //     world
        //         .resources_mut()
        //         .insert(Settings::new(context_provider.message_adapter.clone()));
        // } else {
        //     world.resources_mut().insert(Settings::from_name(
        //         app_name,
        //         context_provider.message_adapter.clone(),
        //     ));
        // };

        self
    }

    /// Builder method to define a widget creation function that returns a widget as root of the ui's shell.
    ///
    /// An ui can only be set once. If the method is multiple called, the last set ui will be used.
    pub fn ui_btx<W, F>(mut self, _ui: F) -> Self
    where
        W: Widget + 'static,
        F: FnOnce(&mut BuildContext) -> W + 'static,
    {
        self
    }

    /// Creates a new shell with the given builder settings.
    pub fn build(self) -> Shell {
        let mut shell = Shell {
            world: self.world,
            frame_buffer: vec![0; self.size.0 as usize * self.size.1 as usize * 4],
            size: self.size,
            mouse_position: Point::default(),
            mouse_button_states: (false, false, false),
            dummy_pos: (0, 0),
        };

        insert_default_resources(&mut shell);

        shell
    }
}

// [START] Helpers

// Insert default resources to the shells world.
pub fn insert_default_resources(shell: &mut Shell) {
    shell.world.insert_resource(RenderContext2D::new(
        shell.size.0 as f64,
        shell.size.1 as f64,
    ));

    // services
    // font_manager
    // application
    // theme manager
}

/// Insert the default set of ECS systems to the world.
pub fn insert_default_systems(world: &mut World<Tree>) {
    // world.register_init_system(InitSystem::new(context_provider.clone()));

    // world.register_cleanup_system(CleanupSystem::new(context_provider.clone()));

    // world
    //     .create_system(EventStateSystem::new(
    //         context_provider.clone(),
    //         RefCell::new(vec![]),
    //     ))
    //     .with_priority(0)
    //     .build();

    // world
    //     .create_system(LayoutSystem::new(context_provider.clone()))
    //     .with_priority(1)
    //     .build();

    // world
    //     .create_system(PostLayoutStateSystem::new(context_provider.clone()))
    //     .with_priority(2)
    //     .build();

    // world
    //     .create_system(RenderSystem::new(context_provider.clone()))
    //     .with_priority(3)
    //     .build();
}

// [END] Helpers
