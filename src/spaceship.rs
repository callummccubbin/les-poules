use std::f32::consts::{FRAC_1_SQRT_2, PI};

use crate::movement::{AngularVelocity, Velocity};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_mod_picking::PickableBundle;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0., -0.5, 0.);
const STARTING_VELOCITY: Vec3 = Vec3::new(0., 0., 0.5);
const STARTING_ANGULAR_VELOCITY: Vec3 = Vec3::new(0., 1., 0.);

const TOPDOWN_TRANSFORMS: [Transform; 6] = [
    // Upwards PY
    Transform {
        translation: Vec3::new(0.0, -1.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    },
    //PX
    Transform {
        translation: Vec3::new(-1.0, 0.0, 0.0),
        rotation: Quat::from_xyzw(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0, 0.0),
        scale: Vec3::ONE,
    },
    //NX
    Transform {
        translation: Vec3::new(1.0, 0.0, 0.0),
        rotation: Quat::from_xyzw(0.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        scale: Vec3::ONE,
    },
    //PZ
    Transform {
        translation: Vec3::new(0.0, 0.0, -1.0),
        rotation: Quat::from_xyzw(0.5, 0.5, 0.5, 0.5),
        scale: Vec3::ONE,
    },
    //NZ
    Transform {
        translation: Vec3::new(0.0, 0.0, 1.0),
        rotation: Quat::from_xyzw(-0.5, -0.5, 0.5, 0.5),
        scale: Vec3::ONE,
    },
    // Downwards NY
    Transform {
        translation: Vec3::new(0.0, 1.0, 0.0),
        rotation: Quat::from_xyzw(1., 0., 0., 0.),
        scale: Vec3::ONE,
    },
];

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    angular_velocity: AngularVelocity,
    model: PbrBundle,
}

#[derive(Bundle)]
struct SpaceshupBundle {
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

fn spawn_spaceship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let custom_texture_handle: Handle<Image> = asset_server.load("placeholder_blocks.png");

    let cube_mesh_handle_side: Handle<Mesh> = meshes.add(create_cube_mesh([0.0, 0.1]));
    let cube_mesh_handle_top: Handle<Mesh> = meshes.add(create_cube_mesh([0.0, 0.0]));
    let cube_mesh_handle_bottom: Handle<Mesh> = meshes.add(create_cube_mesh([0.0, 0.2]));

    println!(
        "{}",
        Quat::from_rotation_z(PI / 2.0).mul_quat(Quat::from_xyzw(
            -FRAC_1_SQRT_2,
            0.0,
            0.0,
            FRAC_1_SQRT_2
        ))
    );

    commands
        .spawn(SpaceshupBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            angular_velocity: AngularVelocity {
                value: STARTING_ANGULAR_VELOCITY,
            },
            model: SceneBundle {
                scene: asset_server.load("Chicken.glb#Scene0"),
                transform: Transform::from_translation(STARTING_TRANSLATION)
                    .with_scale(Vec3 {
                        x: 0.005,
                        y: 0.005,
                        z: 0.005,
                    }),
                ..default()
            },
        })
        .insert(Name::new("sex"))
        .insert(UnpickableGLTF);

    for i in 0..6 {
        commands.spawn(SpaceshipBundle {
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            angular_velocity: AngularVelocity {
                value: STARTING_ANGULAR_VELOCITY,
            },
            model: PbrBundle {
                mesh: match i {
                    0 => cube_mesh_handle_top.clone(),
                    1..=4 => cube_mesh_handle_side.clone(),
                    5 => cube_mesh_handle_bottom.clone(),
                    // learn how to handle this error.
                    _ => cube_mesh_handle_top.clone(),
                },
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(custom_texture_handle.clone()),
                    ..default()
                }),
                transform: TOPDOWN_TRANSFORMS[i],
                ..default()
            },
        });
    }
}

fn set_pickable_recursive(
    commands: &mut Commands,
    entity: &Entity,
    mesh_query: &Query<(Entity, &Parent), With<Handle<Mesh>>>,
    children_query: &Query<&Children>,
) {
    for (mesh_entity, mesh_parent) in mesh_query.iter() {
        if mesh_parent.get() == *entity {
            commands
                .entity(mesh_entity)
                .insert(PickableBundle::default());
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
    children_query: Query<&Children>,
) {
    for (entity, name, children) in unpickable_query.iter_mut() {
        println!(" [MODELS] Setting Pickable on {name}");
        set_pickable_recursive(&mut commands, &entity, &mesh_query, &children_query);
        commands.entity(entity).remove::<UnpickableGLTF>();
    }
}

// assume that the textures are organized into a 10x10 grid in the sheet. Not a fantastic way of implementing.
fn create_cube_mesh(texture_pos: [f32; 2]) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            // Each array is an [x, y, z] coordinate in local space.
            // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
            // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
            vec![
                // top (facing towards +y)
                [-0.5, 0.5, -0.5], // vertex with index 0
                [0.5, 0.5, -0.5],  // vertex with index 1
                [0.5, 0.5, 0.5],   // etc. until 23
                [-0.5, 0.5, 0.5],
            ],
        )
        // Set-up UV coordinated to point to the upper (V < 0.5), "dirt+grass" part of the texture.
        // Take a look at the custom image (assets/textures/array_texture.png)
        // so the UV coords will make more sense
        // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                // Assigning the UV coords for the top side.
                [texture_pos[0], texture_pos[1] + 0.1],
                [texture_pos[0], texture_pos[1]],
                [texture_pos[0] + 0.1, texture_pos[1]],
                [texture_pos[0] + 0.1, texture_pos[1] + 0.1],
            ],
        )
        // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
        // the surface.
        // Normals are required for correct lighting calculations.
        // Each array represents a normalized vector, which length should be equal to 1.0.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                // Normals for the top side (towards +y)
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
            ],
        )
        // Create the triangles out of the 24 vertices we created.
        // To construct a square, we need 2 triangles, therefore 12 triangles in total.
        // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
        // by one, in a counter-clockwise order (relative to the position of the viewer, the order
        // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
        // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
        // further examples and the implementation of the built-in shapes.
        .with_indices(Some(Indices::U32(vec![
            0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
        ])))
}
