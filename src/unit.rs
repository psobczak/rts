use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_rapier3d::prelude::{Collider, RigidBody};

pub struct UnitPlugin;

const UNIT_SIZE: f32 = 0.5;

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct UnitCount(usize);

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitCount::default())
            .register_type::<UnitCount>()
            .add_startup_system(spawn_unit);
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
            material: materials.add(Color::rgb(0.7, 0.5, 0.3).into()),
            transform: Transform::from_xyz(0.0, UNIT_SIZE / 2.0, 0.0),
            ..default()
        },
        Collider::cuboid(UNIT_SIZE / 2.0, UNIT_SIZE / 2.0, UNIT_SIZE / 2.0),
        RigidBody::Dynamic,
        Name::from("Unit"),
        Unit,
    ));

    counter.0 += 1;
}
