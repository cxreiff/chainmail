use bevy::prelude::*;

pub mod current_letter;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((current_letter::plugin,));
}
