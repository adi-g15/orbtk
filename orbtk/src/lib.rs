//! The Orbital Widget Toolkit is a cross-platform (G)UI toolkit.

pub mod events {
    //! This module contains all resources to call and handle events.
    pub use orbtk_base::events::*;
}

pub mod utils {
    //! Helper utils and traits for OrbTk.
    pub use orbtk_base::utils::*;
}

pub mod localization {
    //! Localization service that is used to localize the ui.
    pub use orbtk_base::localization::*;
}

// im features excludes the orbclient app and window implementation
#[cfg(not(feature = "im"))]
pub mod orbclient {
    //! App and window implementation for OrbTk based on OrbClient.
    pub use orbtk_orbclient::*;
}

pub mod shell {
    //! Immediate mode user interface (ui) shell for OrbTk.
    pub use orbtk_shell::*;
}

pub mod tinyskia {
    //! 2D software renderer use by OrbTk based on tiny_skia.
    pub use orbtk_tinyskia::*;
}

pub mod widgets {
    //! Integrated default OrbTk widget library with different themes.
    pub use orbtk_widgets::*;
}

pub use self::utils::*;

#[cfg(not(feature = "im"))]
pub use self::orbclient::*;

pub use self::tinyskia::*;
pub use self::widgets::*;
