mod camera;
mod game;
mod ground;
mod selection;
mod unit;

use camera::CameraPlugin;
use game::GamePlugin;
use ground::GroundPlugin;
use selection::SelectionPlugin;
use unit::UnitPlugin;

use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::PresentMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Menu,
    #[default]
    InGame,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RTS".into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(CameraPlugin)
        .add_plugin(UnitPlugin)
        .add_plugin(GroundPlugin)
        .add_plugin(SelectionPlugin)
        .add_plugin(GamePlugin)
        .run();
}
