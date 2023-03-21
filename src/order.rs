use std::collections::VecDeque;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::Collider;

use crate::{
    camera::get_point_on_ground,
    ground::Ground,
    selection::Selectable,
    units::{Unit, UnitState},
    GameState,
};

pub struct OrderPlugin;

#[derive(Debug, Reflect, FromReflect)]
pub enum Order {
    Move(Vec3),
}

#[derive(Component, Deref, DerefMut, Default)]
pub struct Orders(VecDeque<Order>);

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((send_move_order, handle_orders).in_set(OnUpdate(GameState::InGame)));
    }
}

fn send_move_order(
    window: Query<(&Window, With<PrimaryWindow>)>,
    mut units: Query<(&Selectable, &mut Orders)>,
    camera: Query<(&Camera, &GlobalTransform, With<Camera3d>)>,
    ground: Query<(&Collider, With<Ground>)>,
    input: Res<Input<MouseButton>>,
) {
    let (window, _) = window.single();
    let (camera, camera_transform, _) = camera.single();
    let (ground_collider, _) = ground.single();
    for (selectable, mut orders) in &mut units {
        if selectable.is_selected && input.just_pressed(MouseButton::Right) {
            if let Some(target) =
                get_point_on_ground(window, camera, camera_transform, ground_collider)
            {
                orders.push_back(Order::Move(target))
            }
        }
    }
}

fn handle_orders(mut orders: Query<(&mut Orders, &mut Unit)>) {
    for (mut orders, mut unit) in &mut orders {
        if let Some(order) = orders.pop_front() {
            match order {
                Order::Move(destination) => unit.state = UnitState::Moving(destination),
            }
        }
    }
}
