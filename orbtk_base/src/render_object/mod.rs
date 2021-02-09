//! This module contains all render objects used in OrbTk. Render objects are used to define how to draw parts of a widget.

use std::{any::Any, collections::BTreeMap};

use dces::prelude::*;

use crate::{
    application::ContextProvider, prelude::*, render::RenderContext2D, theming::*, tree::*,
    utils::*,
};

pub use self::cursor::*;
pub use self::default::*;
pub use self::font_icon::*;
pub use self::image::*;
pub use self::pipeline::*;
pub use self::rectangle::*;
pub use self::text::*;

mod cursor;
mod default;
mod font_icon;
mod image;
mod pipeline;
mod rectangle;
mod text;

pub trait RenderObject: Any {
    fn render(
        &self,
        rtx: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        context_provider: &ContextProvider,
        theme: &Theme,
        offsets: &mut BTreeMap<Entity, (f64, f64)>,
        debug: bool,
    ) {
        let mut global_position = Point::default();

        if let Some(parent) = ecm.entity_store().parent[&entity] {
            if let Some(offset) = offsets.get(&parent) {
                global_position = Point::new(offset.0, offset.1);
            }
        }

        if let Ok(visibility) = ecm
            .component_store()
            .get::<Visibility>("visibility", entity)
        {
            if *visibility != Visibility::Visible {
                return;
            }
        } else {
            return;
        }

        rtx.begin_path();
        rtx.set_alpha(
            *ecm.component_store()
                .get::<f32>("opacity", entity)
                .unwrap_or(&1.0),
        );

        // Could be unwrap because every widget has the clip property
        let clip = *ecm.component_store().get::<bool>("clip", entity).unwrap();
        if clip {
            if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", entity) {
                rtx.save();
                rtx.rect(
                    global_position.x() + bounds.x(),
                    global_position.y() + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                );
                rtx.clip();
            }
        }

        self.render_self(
            &mut Context::new((entity, ecm), &theme, context_provider),
            &global_position,
            rtx,
        );

        let mut global_pos = (0.0, 0.0);

        if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", entity) {
            global_pos = (
                global_position.x() + bounds.x(),
                global_position.y() + bounds.y(),
            );
            offsets.insert(entity, global_pos);
        }

        if let Ok(g_pos) = ecm
            .component_store_mut()
            .get_mut::<Point>("position", entity)
        {
            g_pos.set_x(global_pos.0);
            g_pos.set_y(global_pos.1);
        }

        self.render_children(rtx, entity, ecm, context_provider, theme, offsets, debug);

        rtx.close_path();

        if clip {
            rtx.restore();
        }

        // render debug border for each widget
        if debug {
            if let Ok(bounds) = ecm.component_store().get::<Rectangle>("bounds", entity) {
                rtx.begin_path();
                rtx.set_stroke_style(Brush::from("#0033cc"));
                rtx.stroke_rect(
                    global_position.x() + bounds.x(),
                    global_position.y() + bounds.y(),
                    bounds.width(),
                    bounds.height(),
                );
                rtx.close_path();
            }
        }
    }

    fn render_self(&self, _: &mut Context, _: &Point, _rtx: &mut RenderContext2D) {}

    fn render_children(
        &self,
        rtx: &mut RenderContext2D,
        entity: Entity,
        ecm: &mut EntityComponentManager<Tree>,
        context_provider: &ContextProvider,
        theme: &Theme,
        offsets: &mut BTreeMap<Entity, (f64, f64)>,
        debug: bool,
    ) {
        for index in 0..ecm.entity_store().children[&entity].len() {
            let child = ecm.entity_store().children[&entity][index];

            if let Some(render_object) = context_provider.render_objects.borrow().get(&child) {
                render_object.render(rtx, child, ecm, context_provider, theme, offsets, debug);
            }
        }
    }
}
