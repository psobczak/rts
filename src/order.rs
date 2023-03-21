use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::Collider;

use crate::{
    camera::get_point_on_ground, ground::Ground, selection::Selectable, unit::Unit, GameState,
};

pub struct OrderPlugin;

#[derive(Debug, Reflect, FromReflect)]
pub enum Order {
    Move(Vec3),
}

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(send_move_order.in_set(OnUpdate(GameState::InGame)));
    }
}

fn send_move_order(
    window: Query<(&Window, With<PrimaryWindow>)>,
    mut units: Query<(&mut Unit, &Selectable)>,
    camera: Query<(&Camera, &GlobalTransform, With<Camera3d>)>,
    ground: Query<(&Collider, With<Ground>)>,
    input: Res<Input<MouseButton>>,
) {
    let (window, _) = window.single();
    let (camera, camera_transform, _) = camera.single();
    let (ground_collider, _) = ground.single();
    for (mut unit, selectable) in &mut units {
        if selectable.is_selected && input.just_pressed(MouseButton::Right) {
            if let Some(target) =
                get_point_on_ground(window, camera, camera_transform, ground_collider)
            {
                unit.orders.push_back(Order::Move(target))
            }
        }
    }
}
