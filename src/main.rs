use bevy::input::mouse::MouseMotion;
use bevy::time::Stopwatch;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::events::{Drag, Pointer};
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, ColliderMassProperties},
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode},
};




mod camera;
mod change_mouse_pointer;
mod features;



pub fn main() {
    bevy::app::App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 500.0,
        })
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 0.05,
                substeps: 20,
            },
            ..default()
        })
        .insert_resource(features::DebugTimer::default())
        .insert_resource(features::CameraOrientation::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, render_origin)
        .add_systems(
            Update,
            (
                camera::update_camera_system,
                camera::accumulate_mouse_events_system,
            ),
        )
        .add_systems(Update, change_mouse_pointer::change_mouse_icon)
        .add_systems(Update, features::debug)
        .add_systems(Update, (features::update_camera_orientation, features::handle_cube_drag))
        .run();
}

fn render_origin(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X, Color::RED);
    gizmos.line(Vec3::ZERO, Vec3::Y, Color::GREEN);
    gizmos.line(Vec3::ZERO, Vec3::Z, Color::BLUE);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let translation = Vec3::new(1.0, 2.0, 2.0);
    let focus = Vec3::ZERO;
    let transform = Transform::from_translation(translation).looking_at(focus, Vec3::Y);

    commands.spawn((PbrBundle::default(), PickableBundle::default()));
    commands
        .spawn(Camera3dBundle {
            transform,
            ..default()
        })
        .insert(camera::PanOrbitCamera {
            focus,
            radius: translation.length(),
            ..default()
        })
        .insert(VisibilityBundle::default())
        .with_children(|commands| {
            commands.spawn(DirectionalLightBundle {
                directional_light: DirectionalLight {
                    shadows_enabled: false,
                    illuminance: 1000.0,
                    ..default()
                },
                transform: Transform::from_xyz(-2.5, 2.5, 2.5).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });

    // cube parameters
    let cube_size = 0.25;
    let cube_color = Color::rgb(0.8, 0.7, 0.6);

    // light cube (1 kg)
    commands
        .spawn((
            Collider::cuboid(cube_size * 0.5, cube_size * 0.5, cube_size * 0.5),
            RigidBody::Dynamic,
        ))
        .insert(change_mouse_pointer::Clickable)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(cube_size, cube_size, cube_size))),
            material: materials.add(cube_color),
            transform: Transform::from_xyz(0.5, 0.5, 0.0),
            ..default()
        });

    // heavy cube (10 kg)
    commands
        .spawn((
            Collider::cuboid(cube_size * 0.5, cube_size * 0.5, cube_size * 0.5),
            RigidBody::Dynamic,
        ))
        .insert(change_mouse_pointer::Clickable)
        .insert(ColliderMassProperties::Mass(10.0))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(cube_size, cube_size, cube_size))),
            material: materials.add(cube_color),
            transform: Transform::from_xyz(-0.5, 0.5, 0.0),
            ..default()
        });

    // wall parameters
    let wall_height = 0.075;
    let wall_thickness = 0.075;
    let wall_length = 4.0;
    let wall_color = Color::rgb(0.7, 0.7, 0.7);

    // north wall
    commands
        .spawn(Collider::cuboid(
            (wall_length - wall_thickness) * 0.5,
            wall_height * 0.5,
            wall_thickness * 0.5,
        ))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                wall_length - wall_thickness,
                wall_height,
                wall_thickness,
            ))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz(
                -wall_thickness * 0.5,
                wall_height * 0.5,
                (-wall_length + wall_thickness) * 0.5,
            ),
            ..default()
        });

    // east wall
    commands
        .spawn(Collider::cuboid(
            wall_thickness * 0.5,
            wall_height * 0.5,
            (wall_length - wall_thickness) * 0.5,
        ))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                wall_thickness,
                wall_height,
                wall_length - wall_thickness,
            ))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz(
                (wall_length - wall_thickness) * 0.5,
                wall_height * 0.5,
                -wall_thickness * 0.5,
            ),
            ..default()
        });

    // south wall
    commands
        .spawn(Collider::cuboid(
            (wall_length - wall_thickness) * 0.5,
            wall_height * 0.5,
            wall_thickness * 0.5,
        ))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                wall_length - wall_thickness,
                wall_height,
                wall_thickness,
            ))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz(
                wall_thickness * 0.5,
                wall_height * 0.5,
                (wall_length - wall_thickness) * 0.5,
            ),
            ..default()
        });

    // west wall
    commands
        .spawn(Collider::cuboid(
            wall_thickness * 0.5,
            wall_height * 0.5,
            (wall_length - wall_thickness) * 0.5,
        ))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                wall_thickness,
                wall_height,
                wall_length - wall_thickness,
            ))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz(
                (-wall_length + wall_thickness) * 0.5,
                wall_height * 0.5,
                wall_thickness * 0.5,
            ),
            ..default()
        });

    // floor
    commands
        .spawn(Collider::cuboid(2.0, 0.1, 2.0))
        .insert(SpatialBundle::from_transform(Transform::from_xyz(
            0.0, -0.1, 0.0,
        )))
        .with_children(|commands| {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(4.0, 4.0)),
                material: materials.add(Color::rgba(0.9, 0.9, 0.9, 1.0)),
                transform: Transform::from_xyz(0.0, 0.1, 0.0),
                ..default()
            });
        });

    // additional lights
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(-5.0, 5.0, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 5.0, -5.0),
        ..default()
    });
}

