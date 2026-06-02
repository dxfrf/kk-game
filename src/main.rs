mod camera;
mod input;
mod persistence;
mod player;
mod state;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use persistence::{load_keybinds, load_save, LoadRequest};
use player::Player;
use state::GameState;

fn main() {
    let initial_bindings = load_keybinds().unwrap_or_default();

    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Корявая Игра ©".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.08)))
        .init_state::<GameState>()
        .insert_resource(initial_bindings)
        .init_resource::<LoadRequest>()
        .add_plugins(player::PlayerPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(ui::PauseMenuPlugin)
        .add_systems(Update, handle_load)
        .run();
}

fn handle_load(
    mut req: ResMut<LoadRequest>,
    mut player_q: Query<&mut Transform, With<Player>>,
    mut bindings: ResMut<input::KeyBindings>,
) {
    if !req.0 { return; }
    req.0 = false;
    if let Some(save) = load_save() {
        if let Ok(mut tf) = player_q.get_single_mut() {
            tf.translation.x = save.x;
            tf.translation.y = save.y;
        }
    }
    if let Some(kb) = load_keybinds() {
        *bindings = kb;
    }
}
