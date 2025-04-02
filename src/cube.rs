use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;


#[derive(Component)]
pub struct Clickable;


pub fn change_color_on_click_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(Entity, &mut Handle<StandardMaterial>), With<Clickable>>,
    rapier_context: Res<RapierContext>,
) {
    let window = windows.get_single().unwrap();
    
    if input_mouse.just_pressed(MouseButton::Left) {
        if let Some(cursor_pos) = window.cursor_position() {
            if let Some((camera, camera_transform)) = cameras.iter().next() {
                if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    let ray_origin = ray.origin;
                    let ray_direction = ray.direction;
                    
                    
                    if let Some((entity, _toi)) = rapier_context.cast_ray(
                        ray_origin, 
                        *ray_direction, 
                        f32::MAX, 
                        true, 
                        QueryFilter::default()
                    ) {
                        if let Ok((_, mut material)) = query.get_mut(entity) {
                            let mut rng = rand::thread_rng();
                            *material = materials.add(StandardMaterial {
                                base_color: Color::rgb(rng.gen(), rng.gen(), rng.gen()),
                                ..default()
                            });
                            println!("Mouse clicked on entity: {:?}", entity);
                            println!("mouse position: {:?}", cursor_pos);
                        }
                    }
                }
            }
        }
    }
}
