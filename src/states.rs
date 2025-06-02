use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameStates>();
}

#[derive(Default, States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameStates {
    #[default]
    Loading,
    Playing,
}
