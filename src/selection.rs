use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};
use bevy_rapier3d::prelude::Collider;

use crate::{unit::Unit, GameState};

pub struct SelectionPlugin;

#[derive(Debug)]
/// Vec2 values are in screen position
enum SelectionEvent {
    Start(Vec2),
    Current(Vec2),
    End,
}

#[derive(Component, Default, Debug)]
struct Selection {
    width: f32,
    height: f32,
}

#[derive(Component, Default, Debug, Reflect)]
pub struct Selectable {
    pub is_selected: bool,
}

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectionEvent>()
            .register_type::<Selectable>()
            .add_systems(
                (
                    create_selection_events,
                    start_drawing_selection,
                    draw_selection.run_if(any_with_component::<Selection>()),
                    set_selection_size.run_if(any_with_component::<Selection>()),
                    despawn_selection,
                    select_unit,
                    deselect_unit,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}

fn select_unit(
    window: Query<(&Window, With<PrimaryWindow>)>,
    camera: Query<(&Camera, &GlobalTransform, With<Camera3d>)>,
    mut units: Query<(&Collider, &mut Selectable, With<Unit>)>,
    input: Res<Input<MouseButton>>,
) {
    let (window, _) = window.single();
    let (camera, camera_transform, _) = camera.single();

    for (unit_collider, mut selectable, _) in &mut units {
        if is_single_unit_selectable(camera, camera_transform, window, unit_collider)
            && input.just_pressed(MouseButton::Left)
        {
            selectable.is_selected = true
        }
    }
}

fn is_single_unit_selectable(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
    unit_collider: &Collider,
) -> bool {
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            return unit_collider.intersects_local_ray(ray.origin, ray.direction, 100.0);
        }
    }

    false
}

fn deselect_unit(
    window: Query<(&Window, With<PrimaryWindow>)>,
    camera: Query<(&Camera, &GlobalTransform, With<Camera3d>)>,
    mut units: Query<(&Collider, &mut Selectable, With<Unit>)>,
    input: Res<Input<MouseButton>>,
) {
    let (window, _) = window.single();
    let (camera, camera_transform, _) = camera.single();
    for (unit_collider, mut selectable, _) in &mut units {
        if !is_single_unit_selectable(camera, camera_transform, window, unit_collider)
            && input.just_released(MouseButton::Left)
        {
            selectable.is_selected = false;
        }
    }
}

fn create_selection_events(
    input: Res<Input<MouseButton>>,
    window: Query<(&Window, With<PrimaryWindow>)>,
    mut writer: EventWriter<SelectionEvent>,
) {
    let (window, _) = window.single();
    if let Some(cursor_position) = window.cursor_position() {
        if input.just_pressed(MouseButton::Left) {
            writer.send(SelectionEvent::Start(Vec2::new(
                cursor_position.x,
                cursor_position.y,
            )));
        }

        if input.pressed(MouseButton::Left) && !input.just_released(MouseButton::Left) {
            writer.send(SelectionEvent::Current(Vec2::new(
                cursor_position.x,
                cursor_position.y,
            )))
        }

        if input.just_released(MouseButton::Left) {
            writer.send(SelectionEvent::End);
        }
    }
}

fn start_drawing_selection(
    mut reader: EventReader<SelectionEvent>,
    mut commands: Commands,
    camera: Query<(&GlobalTransform, &Camera, With<Camera2d>)>,
) {
    for event in reader.iter() {
        if let SelectionEvent::Start(start) = event {
            let (transform, camera, _) = camera.single();
            if let Some(position) = Camera::viewport_to_world_2d(camera, transform, *start) {
                commands.spawn((
                    Selection::default(),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.2, 0.8, 0.5, 0.4),
                            anchor: Anchor::TopLeft,
                            custom_size: Some(Vec2::ZERO),
                            ..default()
                        },
                        transform: Transform::from_xyz(position.x, position.y, 0.0),
                        ..default()
                    },
                    Name::from("Selection"),
                ));
            }
        }
    }
}

fn despawn_selection(
    mut commands: Commands,
    mut reader: EventReader<SelectionEvent>,
    selection_query: Query<(Entity, With<Selection>)>,
) {
    for event in reader.iter() {
        if let SelectionEvent::End = event {
            let (selection, _) = selection_query.single();
            commands.entity(selection).despawn_recursive()
        }
    }
}

fn draw_selection(
    mut reader: EventReader<SelectionEvent>,
    mut selection_query: Query<(&Selection, &mut Sprite)>,
) {
    let (selection, mut sprite) = selection_query.single_mut();
    for event in reader.iter() {
        if let SelectionEvent::Current(_) = event {
            sprite.custom_size = Some(Vec2::new(selection.width, selection.height));
        }
    }
}

fn set_selection_size(
    mut selection: Query<(&mut Selection, &GlobalTransform)>,
    mut reader: EventReader<SelectionEvent>,
    camera: Query<(&GlobalTransform, &Camera, With<Camera2d>)>,
) {
    let (mut selection, transform) = selection.single_mut();
    for event in reader.iter() {
        if let SelectionEvent::Current(current) = event {
            let spawn_point = transform.translation();
            let (camera_transform, camera, _) = camera.single();
            if let Some(position) = Camera::viewport_to_world_2d(camera, camera_transform, *current)
            {
                let width_diff = spawn_point.x - position.x;
                let height_diff = spawn_point.y - position.y;

                selection.width = -width_diff;
                selection.height = height_diff;
            }
        }
    }
}

// TODO: Send movement order as event maybe?
