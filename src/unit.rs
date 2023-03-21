use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

use crate::{order::Order, selection::Selectable, GameState};

pub struct UnitPlugin;

const UNIT_SIZE: f32 = 0.5;

const NORMAL_COLOR: Color = Color::rgba(0.9, 0.6, 0.1, 1.0);
const HIGHLIHT_COLOR: Color = Color::rgba(0.9, 0.8, 0.5, 0.9);

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Unit>()
            .add_startup_system(spawn_unit)
            .add_systems(
                (handle_highlight, handle_orders, move_units).in_set(OnUpdate(GameState::InGame)),
            );
    }
}
#[derive(Default, Reflect)]

pub enum UnitState {
    Moving(Vec3),
    #[default]
    Idle,
}

#[derive(Component, Default, Reflect)]
pub struct Unit {
    pub orders: VecDeque<Order>,
    pub state: UnitState,
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

fn handle_orders(mut units: Query<(&mut Unit)>) {
    for mut unit in &mut units {
        if let Some(order) = unit.orders.pop_front() {
            match order {
                Order::Move(destination) => unit.state = UnitState::Moving(destination),
            }
        }
    }
}

fn move_units(mut units: Query<(&Unit, &mut Transform)>, time: Res<Time>) {
    for (unit, mut transform) in &mut units {
        if let UnitState::Moving(destination) = unit.state {
            transform.translation +=
                Vec3::new(destination.x, UNIT_SIZE / 2.0, destination.z) * time.delta_seconds();
        }
    }
}
