use bevy::prelude::*;

pub mod debug_frame;
pub mod current_letter;
pub mod utilities;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((current_letter::plugin,));
}
