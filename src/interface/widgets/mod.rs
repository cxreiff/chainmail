use bevy::prelude::*;

pub mod confetti;
pub mod letter;
pub mod prompt;
pub mod statistics;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((letter::plugin, prompt::plugin, confetti::plugin));
}
