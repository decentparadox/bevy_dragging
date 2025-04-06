use bevy::window::CursorIcon;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Dragged {
    pub start_position: Vec3,
    pub start_cursor_position: Vec2,
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
                        if let Ok((_, transform)) = param_set.p0().get_mut(entity) {
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