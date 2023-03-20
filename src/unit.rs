use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::{ground::Ground, selection::Selectable, GameState};

pub struct UnitPlugin;

const UNIT_SIZE: f32 = 0.5;

const NORMAL_COLOR: Color = Color::rgba(0.9, 0.6, 0.1, 1.0);
const HIGHLIHT_COLOR: Color = Color::rgba(0.9, 0.8, 0.5, 0.9);

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct UnitCount(usize);

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitCount::default())
            .register_type::<UnitCount>()
            .add_startup_system(spawn_unit)
            .add_systems((handle_highlight, move_unit).in_set(OnUpdate(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Unit;

fn spawn_unit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut counter: ResMut<UnitCount>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::new(UNIT_SIZE).into()),
            material: materials.add(NORMAL_COLOR.into()),
            transform: Transform::from_xyz(0.0, UNIT_SIZE / 2.0, 0.0),
            ..default()
        },
        Collider::cuboid(UNIT_SIZE / 2.0, UNIT_SIZE / 2.0, UNIT_SIZE / 2.0),
        RigidBody::Dynamic,
        Name::from("Unit"),
        Unit,
        Selectable::default(),
    ));

    counter.0 += 1;
}

fn handle_highlight(
    units: Query<(With<Unit>, &Handle<StandardMaterial>, &Selectable)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (_, handle, selectable) in &units {
        if let Some(material) = materials.get_mut(handle) {
            if selectable.is_selected {
                material.base_color = HIGHLIHT_COLOR;
            } else {
                material.base_color = NORMAL_COLOR;
            }
        }
    }
}

fn move_unit(
    mut units: Query<(&mut Transform, With<Unit>, &Selectable)>,
    time: Res<Time>,
    window: Query<(&Window, With<PrimaryWindow>)>,
    camera: Query<(&Camera, &GlobalTransform, With<Camera3d>)>,
    ground: Query<( &Collider, With<Ground>)>,
    input: Res<Input<MouseButton>>,
) {
    let (window, _) = window.single();
    for (mut transform, _, selectable) in &mut units {
        if selectable.is_selected && input.just_pressed(MouseButton::Right) {
            let (camera, camera_transform, _) = camera.single();
            if let Some(cursor_position) = window.cursor_position() {
                if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    let (ground_collider, _) = ground.single();
                    if let Some(toi) =
                        ground_collider.cast_local_ray(ray.origin, ray.direction, 100.0, true)
                    {
                        let target = ray.origin + ray.direction * toi;
                        transform.translation = Vec3::new(target.x, UNIT_SIZE / 2.0, target.z)
                    }
                }
            }
        }
    }
}
