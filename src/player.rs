use bevy::prelude::*;
use crate::{input::KeyBindings, state::GameState};

pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: f32 = 32.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn player_movement(
    mut player_q: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    bindings: Res<KeyBindings>,
    time: Res<Time>,
) {
    let Ok(mut tf) = player_q.get_single_mut() else { return };
    let mut dir = Vec2::ZERO;
    if keys.pressed(bindings.up)    { dir.y += 1.0; }
    if keys.pressed(bindings.down)  { dir.y -= 1.0; }
    if keys.pressed(bindings.left)  { dir.x -= 1.0; }
    if keys.pressed(bindings.right) { dir.x += 1.0; }
    if dir != Vec2::ZERO {
        tf.translation += (dir.normalize() * PLAYER_SPEED * time.delta_secs()).extend(0.0);
    }
}
