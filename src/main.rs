mod debug;
mod spaceship;
mod movement;
mod camera;

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1.0,
        })

        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        //.add_plugins(DebugPlugin)

        .add_plugins(DefaultPickingPlugins)

        .add_plugins(DefaultPlugins)
        .run();
}




