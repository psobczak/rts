use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::{order::OrderEvent, selection::Selectable, GameState};

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
            .add_systems((handle_highlight, move_units).in_set(OnUpdate(GameState::InGame)));
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

fn move_units(
    mut reader: EventReader<OrderEvent>,
    mut units: Query<(&mut Transform, Entity, With<Unit>)>,
    time: Res<Time>,
) {
    for event in reader.iter() {
        if let OrderEvent::Move((unit, destination)) = event {
            if let Ok((mut transform, _, _)) = units.get_mut(*unit) {
                transform.translation = Vec3::new(destination.x, UNIT_SIZE / 2.0, destination.z);
            }
        }
    }
}
