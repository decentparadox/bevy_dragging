use bevy::input::mouse::MouseMotion;
use bevy::time::Stopwatch;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::events::{Drag, Pointer};
use bevy_rapier3d::dynamics::RigidBody;

use crate::change_mouse_pointer;

#[derive(Resource, Default)]
pub struct DebugTimer {
    stopwatch: Stopwatch,
}


#[derive(Resource, Default)]
pub struct CameraOrientation {
    pub forward: Vec3,
    pub right: Vec3,
}


pub fn debug(
    positions: Query<&Transform, With<RigidBody>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut debug_timer: ResMut<DebugTimer>,
    time: Res<Time>,
) {
    debug_timer.stopwatch.tick(time.delta());

    if debug_timer.stopwatch.elapsed_secs() >= 10.0 {
        let window = windows.get_single().unwrap();
        println!(
            "Window Width: {}, Window Height: {}",
            window.width(),
            window.height()
        );
        for transform in positions.iter() {
            println!(
                "Box Coordinates:{} {}",
                transform.translation.x, transform.translation.y
            );
        }
        for ev in evr_motion.read() {
            println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        }
        debug_timer.stopwatch.reset();
    }
}

pub fn update_camera_orientation(
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

pub fn handle_cube_drag(
    mut drag_events: EventReader<Pointer<Drag>>,
    mut query: Query<&mut Transform, With<change_mouse_pointer::Clickable>>,
    camera_orientation: Res<CameraOrientation>,
    keys: Res<ButtonInput<KeyCode>>,
    
) {
    for event in drag_events.read() {
        if let Ok(mut transform) = query.get_mut(event.target) {
            let dx = event.delta.x / 500.0;
            let dz = event.delta.y / 500.0;
            if !keys.pressed(KeyCode::ControlLeft) {
                transform.rotate_local_y(event.delta.x / 250.0);
            }          
            transform.translation += camera_orientation.right * dx;
            transform.translation -= camera_orientation.forward * dz;
        }
    }
}


