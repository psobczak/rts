mod movement;
mod setup;

use bevy::prelude::*;

use self::{movement::UnitMovementPlugin, setup::UnitSetupPlugin};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Unit>()
            .add_plugin(UnitMovementPlugin)
            .add_plugin(UnitSetupPlugin);
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
    pub state: UnitState,
}
