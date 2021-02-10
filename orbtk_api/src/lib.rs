/*!
   API crate that provides base api and elements for OrbTk like widgets basis.
*/

#[macro_use]
extern crate derive_more;

pub(crate) use orbtk_proc_macros as proc_macros;
pub(crate) use orbtk_render::prelude as render;
pub(crate) use orbtk_shell_old::prelude as shell;
pub(crate) use orbtk_utils::prelude as utils;

pub mod application;
#[macro_use]
pub mod events;
pub mod layout;
pub mod localization;
pub mod prelude;
pub mod properties;
pub mod render_object;
pub mod services;
pub mod systems;
pub mod theming;
pub mod tree;
pub mod widget_base;

#[macro_use]
pub mod macros;
