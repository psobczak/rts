use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::{Collider, Sensor};

use crate::GameState;

pub struct CameraPlugin;

const CAMERA_MIN_HEIGHT: f32 = 3.0;
const CAMERA_MAX_HEIGHT: f32 = 15.0;
const CAMERA_START_HEIGHT: f32 = 7.0;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(restrict_camera)
            .add_systems((restrict_camera,).in_set(OnUpdate(GameState::InGame)))
            .add_system(set_cursor_as_confined.in_schedule(OnEnter(GameState::InGame)))
            .add_system(release_cursor.in_schedule(OnEnter(GameState::Menu)));
    }
}

fn set_cursor_as_confined(mut window: Query<(&mut Window, With<PrimaryWindow>)>) {
    let (mut window, _) = window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Confined
}

fn release_cursor(mut window: Query<(&mut Window, With<PrimaryWindow>)>) {
    let (mut window, _) = window.single_mut();
    window.cursor.grab_mode = CursorGrabMode::None
}

// fn test_cast_ray(
//     camera_2d: Query<(&GlobalTransform, &Collider, With<Camera3d>)>,
//     ground: Query<(&GlobalTransform, &Collider, With<Unit>)>,
// ) {
//     let (camera_2d_transform, camera_collider, _) = camera_2d.single();
//     let (ground_tranform, collider, _) = ground.single();

//     // info!("ray_origin: {}", camera_2d_transform.translation());
//     // info!("ray_dir: {}", ground_tranform.translation());

//     let xd = camera_collider.cast_local_ray(
//         camera_2d_transform.translation(),
//         ground_tranform.translation(),
//         1000.0,
//         true,
//     );

//     // info!("{:?}", xd);
// }

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
            transform: Transform::from_xyz(-2.0, 2.5, 5.0),
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
        Collider::default(),
        Sensor,
    ));
}
