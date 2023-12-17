use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
const CAMERA_DISTANCE: f32 = 100.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE / 4., -CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Z),
         ..default()
    });
}