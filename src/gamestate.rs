use bevy::prelude::*;
use bevy::app::AppExit;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Startup,
    Playing,
    GameOver,
}

pub struct Score {
    pub value: i32,
    pub max: i32,
}

pub struct CurrentLevel {
    pub name: String,
}

pub struct PerfDebug {
    pub spotlight_updates: i32,
}

pub fn startgame_keyboard(mut state: ResMut<State<GameState>>, mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}