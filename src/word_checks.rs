use bevy::prelude::*;

use crate::{interface::widgets::prompt::Prompt, scene::spawning::Star};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(submitted_word_observer);
}

#[derive(Event)]
pub struct SubmittedWord;

fn submitted_word_observer(
    _trigger: Trigger<SubmittedWord>,
    mut commands: Commands,
    prompt: Res<Prompt>,
    word_cubes: Query<(Entity, &Star)>,
) {
    for (entity, word_cube) in &word_cubes {
        if word_cube.word == prompt.text {
            // TODO: activate effect, check for completions.
            commands.entity(entity).despawn();
        }
    }
}
