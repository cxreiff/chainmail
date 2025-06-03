use bevy::prelude::*;

pub mod draw;
pub mod layout;
pub mod utilities;
pub mod widgets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((draw::plugin, widgets::plugin));
}
