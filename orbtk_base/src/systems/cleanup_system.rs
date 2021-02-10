use dces::prelude::*;

use crate::{theming::Theme, tree::Tree, widget_base::Context};

/// Handles the inner cleanup while window is closing.
pub struct CleanupSystem;

impl CleanupSystem {
    /// Creates a new cleanup system.
    pub fn new() -> Self {
        CleanupSystem
    }
}

impl System<Tree> for CleanupSystem {
    fn run(&self, ecm: &mut EntityComponentManager<Tree>, res: &mut Resources) {
        // let mut shell = self.shell.borrow_mut();
        let root = ecm.entity_store().root();
        let theme = ecm
            .component_store()
            .get::<Theme>("theme", root)
            .unwrap()
            .clone();

        let mut dirty_index = 0;

        loop {
            if dirty_index
                >= ecm
                    .component_store()
                    .get::<Vec<Entity>>("dirty_widgets", root)
                    .unwrap()
                    .len()
            {
                break;
            }

            let skip = false;

            let widget = *ecm
                .component_store()
                .get::<Vec<Entity>>("dirty_widgets", root)
                .unwrap()
                .get(dirty_index)
                .unwrap();

            let mut keys = vec![];

            if !skip {
                let mut ctx = Context::new((widget, ecm), &theme);

                if let Some(state) = self.context_provider.states.borrow_mut().get_mut(&widget) {
                    state.cleanup(&mut ctx, res);
                }

                keys.append(&mut ctx.new_states_keys());
            }

            dirty_index += 1;
        }
    }
}
