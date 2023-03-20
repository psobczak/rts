use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    input::mouse::MouseWheel,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::{Collider, Sensor};

use crate::GameState;

pub struct CameraPlugin;

const CAMERA_MIN_HEIGHT: f32 = 3.0;
const CAMERA_MAX_HEIGHT: f32 = 16.0;
const CAMERA_START_HEIGHT: f32 = 7.0;
const CAMERA_MOVE_SPEED: f32 = 50.0;

const ZOOM_SPEED: f32 = 10.0;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_camera, set_default_cursor_position))
            .add_systems(
                (move_camera, zoom, restrict_camera)
                    .chain()
                    .in_set(OnUpdate(GameState::InGame)),
            )
            .add_system(set_cursor_as_confined.in_schedule(OnEnter(GameState::InGame)))
            .add_system(release_cursor.in_schedule(OnEnter(GameState::Menu)));
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

fn move_camera(
    window: Query<(&Window, With<PrimaryWindow>)>,
    mut camera: Query<(&mut Transform, With<Camera3d>)>,
    time: Res<Time>,
) {
    let (window, _) = window.single();
    if let Some(cursor_position) = window.cursor_position() {
        let (mut transform, _) = camera.single_mut();

        if cursor_position.x >= window.width() - 1.0 {
            transform.translation.x += CAMERA_MOVE_SPEED * time.delta_seconds()
        }

        if cursor_position.x <= 1.0 {
            transform.translation.x -= CAMERA_MOVE_SPEED * time.delta_seconds()
        }

        if cursor_position.y >= window.height() - 1.0 {
            transform.translation.z -= CAMERA_MOVE_SPEED * time.delta_seconds()
        }

        if cursor_position.y <= 1.0 {
            transform.translation.z += CAMERA_MOVE_SPEED * time.delta_seconds()
        }
    }
}

fn restrict_camera(mut camera: Query<(&mut Transform, With<Camera3d>)>) {
    let (mut transform, _) = camera.single_mut();
    if transform.translation.y <= CAMERA_MIN_HEIGHT {
        transform.translation.y = CAMERA_MIN_HEIGHT
    }

    if transform.translation.y >= CAMERA_MAX_HEIGHT {
        transform.translation.y = CAMERA_MAX_HEIGHT
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 30_000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, CAMERA_START_HEIGHT, 0.0)
                .with_rotation(Quat::from_rotation_x(-120.0_f32.to_radians())),
            ..default()
        },
        Name::from("Directional Light"),
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_START_HEIGHT, 0.0)
                .with_rotation(Quat::from_rotation_x(-45.0_f32.to_radians())),
            camera: Camera {
                order: 0,
                ..default()
            },
            ..default()
        },
        Name::from("Camera 3d"),
        Collider::default(),
        Sensor,
    ));

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
        Name::from("Camera 2d"),
    ));
}

fn zoom(
    mut camera: Query<(&mut Transform, With<Camera3d>)>,
    mut reader: EventReader<MouseWheel>,
    time: Res<Time>,
) {
    for event in reader.iter() {
        let (mut transform, _) = camera.single_mut();
        if event.y == -1.0 {
            transform.translation.y += time.delta_seconds() * ZOOM_SPEED
        }

        if event.y == 1.0 {
            transform.translation.y -= time.delta_seconds() * ZOOM_SPEED
        }
    }
}
