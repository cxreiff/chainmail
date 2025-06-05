use bevy::prelude::*;

pub mod letter;
pub mod prompt;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((letter::plugin, prompt::plugin));
}
