mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::{prelude::*, window::{PresentMode, WindowResolution}};
use bevy_mod_picking::{debug::DebugPickingMode, DefaultPickingPlugins};
use camera::CameraPlugin;
// use custom_mod_picking::CustomPickingPlugins;
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
        //.add_plugins(MovementPlugin)
        //.add_plugins(DebugPlugin)
        //.add_systems(Startup, disable_picking_debug)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BEST GAME EVAR".to_string(),
                        present_mode: PresentMode::Fifo,
                        cursor: bevy::window::Cursor {
                            icon: CursorIcon::Crosshair,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .run();
}

fn disable_picking_debug(mut pik_debug: ResMut<NextState<DebugPickingMode>>) {
    pik_debug.set(DebugPickingMode::Disabled);
}
