#![crate_name = "orbtk"]
#![crate_type = "lib"]

pub mod api {
    pub use orbtk_api::*;
}

pub mod localization {
    pub use orbtk_api::localization::*;
}

pub mod proc_macros {
    pub use orbtk_proc_macros::*;
}

pub mod render {
    pub use orbtk_render::*;
}

pub mod shell_old {
    pub use orbtk_shell_old::*;
}

pub mod shell {
    //! Immediate mode user interface (ui) shell for OrbTk.
    pub use orbtk_shell::*;
}

pub mod orbclient {
    pub use orbtk_orbclient::*;
}

pub mod theming {
    pub use orbtk_api::theming::*;
}

pub mod themes {
    pub use orbtk_widgets::themes::*;
}
pub mod tree {
    //! This module provides a index (entity) based tree structure compatible to the [DCES](https://gitlab.redox-os.org/redox-os/dces-rust).
    pub use orbtk_api::tree::*;
}

pub mod utils {
    pub use orbtk_utils::*;
}

pub mod widgets {
    pub use orbtk_widgets::*;
}

pub mod prelude;
