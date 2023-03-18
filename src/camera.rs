use bevy::{
    core_pipeline::clear_color::ClearColorConfig, input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

pub struct CameraPlugin;

const CAMERA_MIN_HEIGHT: f32 = 3.0;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FlyCameraPlugin)
            .add_startup_system(spawn_camera)
            .add_system(toggle_button_system.run_if(input_just_pressed(KeyCode::T)))
            .add_system(restrict_camera);
    }
}

fn restrict_camera(mut camera: Query<(&mut Transform, With<FlyCamera>)>) {
    let (mut transform, _) = camera.single_mut();
    if transform.translation.y <= CAMERA_MIN_HEIGHT {
        transform.translation.y = CAMERA_MIN_HEIGHT
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
            transform: Transform::default()
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
        FlyCamera::default(),
        Name::from("Camera 3d"),
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

// Press "T" to toggle keyboard+mouse control over the camera
fn toggle_button_system(mut query: Query<&mut FlyCamera>) {
    for mut options in query.iter_mut() {
        println!("Toggled FlyCamera enabled!");
        options.enabled = !options.enabled;
    }
}
