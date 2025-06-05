use bevy::prelude::*;

pub mod current_letter;
pub mod prompt;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((current_letter::plugin, prompt::plugin));
}
