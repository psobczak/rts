use bevy::prelude::*;

use crate::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pause_game.run_if(in_state(GameState::InGame)))
            .add_system(resume_game.run_if(in_state(GameState::Menu)));
    }
}

fn pause_game(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu)
    }
}

fn resume_game(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::InGame)
    }
}
