use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use crate::movement::{Velocity, AngularVelocity};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., -20.,);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 0.5,);
const STARTING_ANGULAR_VELOCITY: Vec3 = Vec3::new(0., 1., 0.,);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    angular_velocity: AngularVelocity,
    model: SceneBundle,
}

pub struct SpaceshipPlugin;

#[derive(Component)]
pub struct UnpickableGLTF;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
        app.add_systems(Startup, make_gltf_scene_pickable.after(spawn_spaceship));
    }
}

fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    for ang in 0..4 {
        commands.spawn(SpaceshipBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            angular_velocity: AngularVelocity {
                value: STARTING_ANGULAR_VELOCITY,
            },
            model: SceneBundle {
                scene: asset_server.load("untitled.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION)
                    .with_scale(Vec3 { x: 10., y: 10., z: 10. })
                    .with_rotation(Quat::from_rotation_y(ang as f32 * PI / 2.)),
                ..default()
            }
        }).insert(Name::new("sex")).insert(UnpickableGLTF);
    }
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        angular_velocity: AngularVelocity {
            value: STARTING_ANGULAR_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load("untitled.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION)
                .with_scale(Vec3 { x: 10., y: 10., z: 10. })
                .with_rotation(Quat::from_rotation_z(-PI / 2.)),
            ..default()
        }
    });
}

fn set_pickable_recursive(
    commands: &mut Commands,
    entity: &Entity,
    mesh_query: &Query<(Entity, &Parent), With<Handle<Mesh>>>,
    children_query: &Query<&Children>,
  ) {
    for (mesh_entity, mesh_parent) in mesh_query.iter(){
      if mesh_parent.get() == *entity {
        commands.entity(mesh_entity).insert(PickableBundle::default());
      }
    }
  
    if let Ok(children) = children_query.get(*entity) {
      for child in children.iter() {
        set_pickable_recursive(commands, child, mesh_query, children_query);
      }
    }
  }

fn make_gltf_scene_pickable(
    mut commands: Commands,
    mut unpickable_query: Query<(Entity, &Name, &Children), With<UnpickableGLTF>>,  
    mesh_query: Query<(Entity, &Parent), With<Handle<Mesh>>>,
    children_query: Query<&Children>
){

for (entity, name, children) in unpickable_query.iter_mut(){
println!(" [MODELS] Setting Pickable on {name}");
set_pickable_recursive(&mut commands, &entity, &mesh_query, &children_query);
commands.entity(entity).remove::<UnpickableGLTF>();
}
}