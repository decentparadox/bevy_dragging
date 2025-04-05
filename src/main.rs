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

mod camera;
mod cube;

#[derive(Resource, Default)]
struct DebugTimer {
    stopwatch: Stopwatch,
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
        .add_systems(Update, cube::change_mouse_icon)
        .add_systems(Update, debug)
        // .add_systems(Update, inspector_ui)
        // Uncomment to draw the global origin
        //.add_systems(Update, render_origin)
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
        .insert(cube::Clickable)
        .insert(ColliderMassProperties::Mass(1.0))
        .insert((PickableBundle::default(), On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.rotate_local_y(drag.delta.x / 50.0);
            transform.translation.x += drag.delta.x / 1000.0;
            transform.translation.z += drag.delta.y / 1000.0;
        })))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(cube_size, cube_size, cube_size))),
            material: materials.add(cube_color),
            transform: Transform::from_xyz(0.5, 0.5, 0.0),
            ..default()
        });
        // .insert( On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
        //     print!("hello");
        // }));


        // commands.spawn((
        //     PbrBundle { 
        //         mesh: meshes.add(Mesh::from(Cuboid::new(cube_size, cube_size, cube_size))),
        //         material: materials.add(cube_color),
        //         transform: Transform::from_xyz(0.5, 0.5, 0.0),
        //         ..default()
        //     },
        //     // These callbacks are run when this entity or its children are interacted with.
        //     // On::<Pointer<Move>>::run(change_hue_with_vertical_move),
        //     // Rotate an entity when dragged:
        //     On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
        //         print!("drag");
        //         transform.rotate_local_y(drag.delta.x / 50.0)
        //     }),
        //     // Despawn an entity when clicked:
        //     On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
        //         target_commands.despawn();
        //     }),
        //     // Send an event when the pointer is pressed over this entity:
        //     // On::<Pointer<Down>>::send_event::<DoSomethingComplex>(),
        // ));



    // heavy cube (10 kg)
    commands
        .spawn((Collider::cuboid(cube_size * 0.5, cube_size * 0.5, cube_size * 0.5), RigidBody::Dynamic))
        .insert(cube::Clickable)
        .insert(ColliderMassProperties::Mass(10.0))
        .insert((PickableBundle::default(), On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.rotate_local_y(drag.delta.x / 50.0);
            transform.translation.x += drag.delta.x / 1000.0;
            transform.translation.z += drag.delta.y / 1000.0;
        })))
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

// [TODO: Needs Cleaning]
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
        // ui.label("Window Width:");
        // ui.horizontal(|ui| {
        //     ui.label("Window Width:");
        // });
        for transform in positions.iter() {
            println!("Box Coordinates:{} {}", transform.translation.x,transform.translation.y);
        }
        debug_timer.stopwatch.reset();
    }
    // for ev in evr_motion.read() {
    //     println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    // }

}



// fn inspector_ui(
//     world: &mut World,
//     positions: Query<&Transform, With<RigidBody>>, 
//     windows: Query<&Window, With<PrimaryWindow>>,
//     mut evr_motion: EventReader<MouseMotion>,
//     mut contexts: EguiContexts
// ) {
//     // let mut egui_context = world
//     // .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
//     // .single(world)
//     // .clone();

//     egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
//         egui::ScrollArea::both().show(ui, |ui| {
//             // equivalent to `WorldInspectorPlugin`
            
//             ui.heading("Debug Info");
            
//             for ev in evr_motion.read() {
//                 let mut delta_x = ev.delta.x;
//                 let mut delta_y = ev.delta.y;
//                 ui.label("Mouse Position: X");
//                 bevy_inspector::ui_for_value(&mut delta_x, ui, world);
//                 ui.label("Mouse Position: Y");
//                 bevy_inspector::ui_for_value(&mut delta_y, ui, world);
//                 println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
//             }
            
//             ui.heading("Entities");
//             bevy_inspector::ui_for_world_entities(world, ui);
//             egui::CollapsingHeader::new("Materials").show(ui, |ui| {
//                 bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
//             });

//         });
//     });
// }
