use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::Collider;

use crate::{
    camera::get_point_on_ground, ground::Ground, selection::Selectable, unit::Unit, GameState,
};

pub struct OrderPlugin;

pub enum OrderEvent {
    Move((Entity, Vec3)),
}

impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrderEvent>()
            .add_system(send_move_order.in_set(OnUpdate(GameState::InGame)));
    }
}

fn send_move_order(
    mut writer: EventWriter<OrderEvent>,
    window: Query<(&Window, With<PrimaryWindow>)>,
    units: Query<(With<Unit>, &Selectable, Entity)>,
    camera: Query<(&Camera, &GlobalTransform, With<Camera3d>)>,
    ground: Query<(&Collider, With<Ground>)>,
    input: Res<Input<MouseButton>>,
) {
    let (window, _) = window.single();
    let (camera, camera_transform, _) = camera.single();
    let (ground_collider, _) = ground.single();
    for (_, selectable, unit) in &units {
        if selectable.is_selected && input.just_pressed(MouseButton::Right) {
            if let Some(target) =
                get_point_on_ground(window, camera, camera_transform, ground_collider)
            {
                writer.send(OrderEvent::Move((unit, target)))
            }
        }
    }
}
