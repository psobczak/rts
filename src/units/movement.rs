use bevy::prelude::*;

use crate::GameState;

use super::{Unit, UnitState};

const UNIT_SPEED: f32 = 5.0;

pub struct UnitMovementPlugin;

impl Plugin for UnitMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_units.in_set(OnUpdate(GameState::InGame)));
    }
}

pub fn move_units(mut units: Query<(&Unit, &mut Transform)>, time: Res<Time>) {
    for (unit, mut transform) in &mut units {
        if let UnitState::Moving(destination) = unit.state {
            transform.look_at(destination, Vec3::X);
            let direction = transform.forward();
            let direction = Vec3::new(direction.x, 0.0, direction.z);
            transform.translation += direction * time.delta_seconds() * UNIT_SPEED;
        }
    }
}
