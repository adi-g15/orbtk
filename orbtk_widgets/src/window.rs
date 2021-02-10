use std::{collections::VecDeque, rc::Rc};

use crate::{
    api::prelude::*, proc_macros::*, render::RenderContext2D, shell::prelude::WindowRequest,
    themes::default::*,
};

// --- KEYS --
pub static STYLE_OldWindow: &str = "OldWindow";
// --- KEYS --

// internal type to handle dirty widgets.
type DirtyWidgets = Vec<Entity>;

#[derive(Clone)]
enum Action {
    OldWindowEvent(WindowEvent),
    FocusEvent(FocusEvent),
}

// The `OldWindowState` handles the OldWindow events.
#[derive(Default, AsAny)]
struct OldWindowState {
    actions: VecDeque<Action>,
    background: Brush,
    title: String,
}

impl OldWindowState {
    fn push_action(&mut self, action: Action) {
        self.actions.push_front(action);
    }

    fn resize(&self, width: f64, height: f64, ctx: &mut Context) {
        OldWindow::bounds_mut(&mut ctx.window()).set_size(width, height);
        OldWindow::constraint_mut(&mut ctx.window()).set_size(width, height);
    }

    fn active_changed(&self, active: bool, ctx: &mut Context) {
        OldWindow::active_set(&mut ctx.widget(), active);

        // if !active {
        //     // remove focus if the OldWindow is not active
        //     if let Some(focused_widget) = ctx.OldWindow().get::<Global>("global").focused_widget {
        //         ctx.OldWindow().get_mut::<Global>("global").focused_widget = None;
        //         if ctx.get_widget(focused_widget).has::<bool>("focused") {
        //             ctx.get_widget(focused_widget).set("focused", false);
        //             ctx.get_widget(focused_widget).update_theme_by_state(false);
        //         }
        //     }
        // }
    }

    fn request_focus(&self, entity: Entity, ctx: &mut Context) {
        let mut focus_state: FocusState = OldWindow::focus_state_clone(&ctx.widget());
        focus_state.request_focus(entity, ctx);
        OldWindow::focus_state_set(&mut ctx.widget(), focus_state);
    }

    fn remove_focus(&self, entity: Entity, ctx: &mut Context) {
        let mut focus_state: FocusState = OldWindow::focus_state_clone(&ctx.widget());
        focus_state.remove_focus(entity, ctx);
        OldWindow::focus_state_set(&mut ctx.widget(), focus_state);
    }

    fn set_background(&mut self, ctx: &mut Context, rtx: &mut RenderContext2D) {
        let background: Brush = ctx.widget().clone("background");
        if let Brush::SolidColor(color) = background {
            rtx.set_background(color);
        };
        self.background = background;
    }
}

impl State for OldWindowState {
    fn init(&mut self, ctx: &mut Context, res: &mut Resources) {
        self.set_background(ctx, res.get_mut::<RenderContext2D>());
        self.title = ctx.widget().clone("title");
    }

    fn update(&mut self, ctx: &mut Context, res: &mut Resources) {
        if self.background != *OldWindow::background_ref(&ctx.widget()) {
            self.set_background(ctx, res.get_mut::<RenderContext2D>());
        }

        let OldWindow = ctx.widget();

        if !self.title.eq(OldWindow::title_ref(&OldWindow)) {
            self.title = OldWindow::title_clone(&OldWindow);
            ctx.send_window_request(WindowRequest::ChangeTitle(self.title.clone()));
        }

        if let Some(action) = self.actions.pop_front() {
            match action {
                Action::OldWindowEvent(window_event) => match window_event {
                    WindowEvent::Resize { width, height } => {
                        self.resize(width, height, ctx);
                    }
                    WindowEvent::ActiveChanged(active) => {
                        self.active_changed(active, ctx);
                    }
                    _ => {}
                },
                Action::FocusEvent(focus_event) => match focus_event {
                    FocusEvent::RequestFocus(entity) => {
                        self.request_focus(entity, ctx);
                    }
                    FocusEvent::RemoveFocus(entity) => {
                        self.remove_focus(entity, ctx);
                    }
                },
            }
        }
    }
}

widget!(
    /// The `OldWindow` widget provides access to the properties of an application OldWindow.
    /// It also contains global properties like keyboard modifier and focused widget.
    ///
    /// **style:** `OldWindow`
    OldWindow<OldWindowState>: ActivateHandler {
        /// Sets or shares the background property.
        background: Brush,

        /// Sets or shares the title property.
        title: String,

        /// Sets or shares the resizeable property.
        resizeable: bool,

        /// Sets or shares the property if this OldWindow should always be on top.
        always_on_top: bool,

        /// Sets or shares the flag if the OldWindow is borderless.
        borderless: bool,

        /// Sets or shares a value that describes if the current OldWindow is active.
        active: bool,

        /// Access the current keyboard state e.g. to check modifiers.
        keyboard_state: KeyboardState,

        /// Access the current OldWindow theme.
        theme: Theme,

        /// Access the current focus state.
        focus_state: FocusState,

        /// Internal property to handle dirty widgets.
        dirty_widgets: DirtyWidgets
    }
);

impl OldWindow {
    fn on_OldWindow_event<H: Fn(&mut StatesContext, WindowEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(WindowEventHandler {
            handler: Rc::new(handler),
        })
    }

    fn on_focus_event<H: Fn(&mut StatesContext, FocusEvent) -> bool + 'static>(
        self,
        handler: H,
    ) -> Self {
        self.insert_handler(FocusEventHandler {
            handler: Rc::new(handler),
        })
    }
}

impl Template for OldWindow {
    fn template(self, id: Entity, _: &mut BuildContext) -> Self {
        self.name("OldWindow")
            .background(colors::BRIGHT_GRAY_COLOR)
            .size(100.0, 100.0)
            .style(STYLE_OldWindow)
            .title("OldWindow")
            .resizeable(false)
            .always_on_top(false)
            .on_OldWindow_event(move |ctx, event| {
                ctx.get_mut::<OldWindowState>(id)
                    .push_action(Action::OldWindowEvent(event));
                true
            })
            .on_focus_event(move |ctx, event| {
                ctx.get_mut::<OldWindowState>(id)
                    .push_action(Action::FocusEvent(event));
                true
            })
    }

    fn render_object(&self) -> Box<dyn RenderObject> {
        RectangleRenderObject.into()
    }

    fn layout(&self) -> Box<dyn Layout> {
        GridLayout::new().into()
    }
}
