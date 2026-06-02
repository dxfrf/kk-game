use bevy::prelude::*;

pub mod keybinds;
pub mod pause;

pub use pause::PauseMenuPlugin;

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy)]
pub enum UiScreen {
    #[default]
    Main,
    Keybinds,
}
