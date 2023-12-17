use bevy::{prelude::*, log};
use bevy::math::f32::Quat;
pub struct MovementPlugin;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

#[derive(Component, Debug)]
pub struct AngularVelocity {
    pub value: Vec3,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
        app.add_systems(Update, update_rotation);
        //app.add_systems(Update, accelerotate);
    }
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.value.x * time.delta_seconds();
        transform.translation.y += velocity.value.y * time.delta_seconds();
        transform.translation.z += velocity.value.z * time.delta_seconds();
    }
}

fn update_rotation(mut query: Query<(&AngularVelocity, &mut Transform)>, time: Res<Time>) {
    for (angular_velocity, mut transform) in query.iter_mut() {
        transform.rotate(
            Quat::from_euler(
                EulerRot::XYZ, 
                angular_velocity.value.x * time.delta_seconds(), 
                angular_velocity.value.y * time.delta_seconds(), 
                angular_velocity.value.z * time.delta_seconds()))
    }   
}

fn accelerotate(mut query: Query<(&mut AngularVelocity)>, time: Res<Time>) {
    for mut angular_velocity in query.iter_mut() {
        angular_velocity.value.y += 1.1 * time.delta_seconds();
    }
}