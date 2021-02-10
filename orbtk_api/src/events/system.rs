use crate::{events::Event, proc_macros::Event};

#[derive(Event)]
pub enum SystemEvent {
    Quit,
}
