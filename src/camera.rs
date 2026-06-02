use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn camera_follow(
    player_q: Query<&Transform, With<Player>>,
    mut cam_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_tf) = player_q.get_single() else { return };
    let Ok(mut cam_tf) = cam_q.get_single_mut() else { return };
    let target = player_tf.translation.truncate().extend(cam_tf.translation.z);
    // Framerate-independent exponential smoothing
    let t = 1.0 - (-12.0_f32 * time.delta_secs()).exp();
    cam_tf.translation = cam_tf.translation.lerp(target, t);
}
