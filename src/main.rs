use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::{events::{Click, Drag, Move, Pointer}, prelude::On};
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, ColliderMassProperties},
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin, TimestepMode},
    render::RapierDebugRenderPlugin
};
use bevy::input::mouse::MouseMotion;
use bevy::time::Stopwatch;
use bevy_inspector_egui::{
    quick::WorldInspectorPlugin,
    bevy_inspector
};
use bevy_egui::{egui, EguiPlugin, EguiContexts};
use bevy_inspector_egui::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy::window::CursorIcon;
use bevy_rapier3d::prelude::*;
use rand::Rng;

mod camera;

#[derive(Component)]
pub struct Clickable;


#[derive(Resource, Default)]
struct DebugTimer {
    stopwatch: Stopwatch,
}

#[derive(Component)]
pub struct Dragged {
    pub start_position: Vec3,
    pub start_cursor_position: Vec2,
}

#[derive(Resource, Default)]
pub struct CameraOrientation {
    pub forward: Vec3,
    pub right: Vec3,
}


pub fn main() {
    bevy::app::App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 500.0,
        })
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed { dt: 0.05, substeps: 20 },
            // Useful settings for debugging
            // physics_pipeline_active: false,
            // query_pipeline_active: false,
            // gravity: Vec3::ZERO,
            ..default()
        })
        .insert_resource(DebugTimer::default())
        .insert_resource(CameraOrientation::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        // .add_plugins(EguiPlugin)
        // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        // Uncomment to show bodies as the physics engine sees them
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, render_origin)
        .add_systems(Update, (camera::update_camera_system, camera::accumulate_mouse_events_system))
        .add_systems(Update, change_mouse_icon)
        .add_systems(Update, debug)
        .add_systems(Update, (update_camera_orientation,handle_cube_drag)) 
        // .add_systems(Update, inspector_ui)
        // Uncomment to draw the global origin
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
    let transform = Transform::from_translation(translation)
        .looking_at(focus, Vec3::Y);

    commands.spawn((
            PbrBundle::default(),           // The raycasting backend works with meshes
            PickableBundle::default(),      // Makes the entity pickable, and adds optional features
        ));
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
                transform: Transform::from_xyz(-2.5, 2.5, 2.5)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });

    // cube parameters
    let cube_size = 0.25;
    let cube_color = Color::rgb(0.8, 0.7, 0.6);

    // light cube (1 kg)
    commands
        .spawn((Collider::cuboid(cube_size * 0.5, cube_size * 0.5, cube_size * 0.5), RigidBody::Dynamic, ))
        .insert(Clickable)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(cube_size, cube_size, cube_size))),
            material: materials.add(cube_color),
            transform: Transform::from_xyz(0.5, 0.5, 0.0),
            ..default()
        });
        // .insert( On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
        //     print!("hello");
        // }));




    // heavy cube (10 kg)
    commands
        .spawn((Collider::cuboid(cube_size * 0.5, cube_size * 0.5, cube_size * 0.5), RigidBody::Dynamic))
        .insert(Clickable)
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
        .spawn(Collider::cuboid((wall_length - wall_thickness) * 0.5, wall_height * 0.5, wall_thickness * 0.5))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(wall_length - wall_thickness, wall_height, wall_thickness))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz(-wall_thickness * 0.5, wall_height * 0.5, (-wall_length + wall_thickness) * 0.5),
            ..default()
        });

    // east wall
    commands
        .spawn(Collider::cuboid(wall_thickness * 0.5, wall_height * 0.5, (wall_length - wall_thickness) * 0.5))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(wall_thickness, wall_height, wall_length - wall_thickness))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz((wall_length - wall_thickness) * 0.5, wall_height * 0.5, -wall_thickness * 0.5),
            ..default()
        });

    // south wall
    commands
        .spawn(Collider::cuboid((wall_length - wall_thickness) * 0.5, wall_height * 0.5, wall_thickness * 0.5))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(wall_length - wall_thickness, wall_height, wall_thickness))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz(wall_thickness * 0.5, wall_height * 0.5, (wall_length - wall_thickness) * 0.5),
            ..default()
        });

    // west wall
    commands
        .spawn(Collider::cuboid(wall_thickness * 0.5, wall_height * 0.5, (wall_length - wall_thickness) * 0.5))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(wall_thickness, wall_height, wall_length - wall_thickness))),
            material: materials.add(wall_color),
            transform: Transform::from_xyz((-wall_length + wall_thickness) * 0.5, wall_height * 0.5, wall_thickness * 0.5),
            ..default()
        });

    // floor
    commands
        .spawn(
            Collider::cuboid(2.0, 0.1, 2.0),
            
        )
        .insert(SpatialBundle::from_transform(Transform::from_xyz(0.0, -0.1, 0.0)))
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

fn debug(
    positions: Query<&Transform, With<RigidBody>>, 
    windows: Query<&Window, With<PrimaryWindow>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut debug_timer: ResMut<DebugTimer>,
    time: Res<Time>,
) {

    debug_timer.stopwatch.tick(time.delta());

    if debug_timer.stopwatch.elapsed_secs() >= 10.0 {
        let window = windows.get_single().unwrap();
        println!("Window Width: {}, Window Height: {}",window.width(), window.height());
        for transform in positions.iter() {
            println!("Box Coordinates:{} {}", transform.translation.x,transform.translation.y);
        }
        for ev in evr_motion.read() {
         println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        }
        debug_timer.stopwatch.reset();
    }


}

fn update_camera_orientation(
    mut camera_orientation: ResMut<CameraOrientation>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let forward = camera_transform.forward();
        let right = forward.cross(Vec3::Y).normalize();

        camera_orientation.forward = Vec3::Y.cross(right).normalize(); // make sure it's orthogonal
        camera_orientation.right = right;
    }
}

fn handle_cube_drag(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut query: Query<&mut Transform, With<Clickable>>,
    camera_orientation: Res<CameraOrientation>,
) {
    for event in drag_events.read() {
        if let Ok(mut transform) = query.get_mut(event.target) {
            let dx = event.delta.x / 250.0;
            let dz = event.delta.y / 250.0;

            transform.translation += camera_orientation.right * dx;
            transform.translation -= camera_orientation.forward * dz;
        }
    }
}


pub fn change_mouse_icon(
    mut commands: Commands,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut windows: Query<&mut bevy::prelude::Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    rapier_context: Res<RapierContext>,
    mut param_set: ParamSet<(
        Query<(Entity, &mut Transform), With<Clickable>>,
        Query<(&mut Transform, &Dragged)>,
    )>,
) {
    let mut primary_window = windows.single_mut();

    if input_mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = primary_window.cursor_position() {
            if let Some((camera, camera_transform)) = cameras.iter().next() {
                if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    let ray_origin = ray.origin;
                    let ray_direction = ray.direction;

                    if let Some((entity, _toi)) = rapier_context.cast_ray(
                        ray_origin,
                        *ray_direction,
                        f32::MAX,
                        true,
                        QueryFilter::default(),
                    ) {
                        if let Ok((_, mut transform)) = param_set.p0().get_mut(entity) {
                             commands.entity(entity).insert(Dragged {
                                start_position: transform.translation,
                                start_cursor_position: cursor_pos,
                            });
                            primary_window.cursor.icon = CursorIcon::Move;
                            println!("Mouse clicked on entity: {:?}", entity);
                            println!("mouse position: {:?}", cursor_pos);
                        } else {
                            primary_window.cursor.icon = CursorIcon::Default;
                        }
                    } else {
                        primary_window.cursor.icon = CursorIcon::Default;
                    }
                }
            }
        }
    } else if input_mouse.just_released(MouseButton::Left) {
        for (entity, _) in param_set.p0().iter() {
            commands.entity(entity).remove::<Dragged>();
        }
        primary_window.cursor.icon = CursorIcon::Default;
    }
}
