use bevy::prelude::*;

mod camera;
mod spawning;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, spawning::plugin));
}
