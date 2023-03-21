use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};
use bevy_rapier3d::prelude::Collider;

use crate::{ground::Ground, GameState};

pub struct CursorPlugin;

#[derive(Component)]
struct MoveMark;

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
/// Coursor postion in world coordinates
struct CursorPosition(Vec3);

#[derive(Component)]
struct MoveSphereDissapearTimer(Timer);

enum CursorEvent {
    OverGround(Vec3),
    OutOfBounds,
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ResourceInspectorPlugin::<CursorPosition>::default())
            .register_type::<CursorPosition>()
            .init_resource::<CursorPosition>()
            .add_event::<CursorEvent>()
            .add_startup_system(set_default_cursor_position)
            .add_system(set_cursor_as_confined.in_schedule(OnEnter(GameState::InGame)))
            .add_system(release_cursor.in_schedule(OnEnter(GameState::Menu)))
            .add_systems(
                (
                    remove_cursor_position_resource.run_if(resource_exists::<CursorPosition>()),
                    update_cursor_position.run_if(resource_exists::<CursorPosition>()),
                    spawn_move_mark.run_if(resource_exists::<CursorPosition>()),
                    add_cursor_position_resource.run_if(not(resource_exists::<CursorPosition>())),
                    handle_cursor_over_ground,
                    decrease_move_mark_alpha,
                )
                    .in_set(OnUpdate(GameState::InGame)),
            );
    }
}

fn remove_cursor_position_resource(mut reader: EventReader<CursorEvent>, mut commands: Commands) {
    for event in reader.iter() {
        if let CursorEvent::OutOfBounds = event {
            commands.remove_resource::<CursorPosition>()
        }
    }
}

fn add_cursor_position_resource(mut reader: EventReader<CursorEvent>, mut commands: Commands) {
    for event in reader.iter() {
        if let CursorEvent::OverGround(translation) = event {
            commands.insert_resource(CursorPosition(*translation))
        }
    }
}

fn handle_cursor_over_ground(
    window: Query<(&Window, With<PrimaryWindow>)>,
    camera: Query<(&Camera, &GlobalTransform, With<Camera3d>)>,
    ground: Query<(&Collider, With<Ground>)>,
    mut writer: EventWriter<CursorEvent>,
) {
    let (window, _) = window.single();
    let (camera, camera_transform, _) = camera.single();
    let (ground_collider, _) = ground.single();

    match get_point_on_ground(window, camera, camera_transform, ground_collider) {
        Some(translation) => writer.send(CursorEvent::OverGround(translation)),
        None => writer.send(CursorEvent::OutOfBounds),
    }
}

fn update_cursor_position(
    mut position: ResMut<CursorPosition>,
    mut reader: EventReader<CursorEvent>,
) {
    for event in reader.iter() {
        if let CursorEvent::OverGround(translation) = event {
            position.0 = *translation
        }
    }
}

fn set_default_cursor_position(mut window: Query<(&mut Window, With<PrimaryWindow>)>) {
    let (mut window, _) = window.single_mut();
    let x = window.width() / 2.0;
    let y = window.height() / 2.0;
    window.set_cursor_position(Some(Vec2::new(x, y)))
}

fn set_cursor_as_confined(mut window: Query<(&mut Window, With<PrimaryWindow>)>) {
    let (mut window, _) = window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Confined
}

fn release_cursor(mut window: Query<(&mut Window, With<PrimaryWindow>)>) {
    let (mut window, _) = window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::None
}

pub fn get_point_on_ground(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
    ground_collider: &Collider,
) -> Option<Vec3> {
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
            if let Some(toi) =
                ground_collider.cast_local_ray(ray.origin, ray.direction, 100.0, true)
            {
                return Some(ray.origin + ray.direction * toi);
            }
        }
    }

    None
}

fn spawn_move_mark(
    mut commands: Commands,
    input: Res<Input<MouseButton>>,
    position: Res<CursorPosition>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if input.just_pressed(MouseButton::Right) {
        commands.spawn((
            Name::from("Move Mark"),
            PbrBundle {
                mesh: meshes.add(
                    shape::UVSphere {
                        radius: 0.1,
                        ..default()
                    }
                    .into(),
                ),
                material: materials.add(Color::YELLOW_GREEN.into()),
                transform: Transform::from_translation(position.0),
                ..default()
            },
            MoveSphereDissapearTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            MoveMark,
        ));
    }
}

fn decrease_move_mark_alpha(
    mut marks: Query<(
        &Handle<StandardMaterial>,
        &mut MoveSphereDissapearTimer,
        With<MoveMark>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (handle, mut timer, _) in &mut marks {
        if let Some(material) = materials.get_mut(handle) {
            timer.0.tick(time.delta());

            if timer.0.just_finished() {
                let last_alpha = material.base_color.a();
                material.base_color.set_a(last_alpha + 0.1);
            }
        }
    }
}
