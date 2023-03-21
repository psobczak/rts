use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::{order::Orders, selection::Selectable, GameState};

use super::Unit;

const UNIT_SIZE: f32 = 0.5;

const NORMAL_COLOR: Color = Color::rgba(0.9, 0.6, 0.1, 1.0);
const HIGHLIHT_COLOR: Color = Color::rgba(0.9, 0.8, 0.5, 0.9);

pub struct UnitSetupPlugin;

impl Plugin for UnitSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_unit)
            .add_systems((handle_highlight,).in_set(OnUpdate(GameState::InGame)));
    }
}

fn spawn_unit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
        Unit::default(),
        Selectable::default(),
        Orders::default(),
    ));
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
