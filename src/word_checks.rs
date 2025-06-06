use bevy::prelude::*;

use crate::{
    constants::{MAC_GREEN_COLOR, MAC_PURPLE_COLOR, MAC_RED_COLOR, MAC_YELLOW_COLOR},
    interface::widgets::{confetti::ConfettiSpawn, prompt::Prompt},
    letters::CurrentLetter,
    scene::spawning::WordCube,
    states::LetterCleared,
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(submitted_word_observer);
}

#[derive(Event)]
pub struct SubmittedWord;

fn submitted_word_observer(
    _trigger: Trigger<SubmittedWord>,
    mut commands: Commands,
    mut prompt: ResMut<Prompt>,
    mut current_letter: ResMut<CurrentLetter>,
    word_cubes: Query<(Entity, &WordCube, &Transform)>,
) {
    for (entity, word_cube, transform) in &word_cubes {
        if word_cube.word == prompt.text {
            // TODO: activate effect, check for completions.
            commands.entity(entity).despawn();
            commands.trigger(ConfettiSpawn {
                position: transform.translation,
                color: color_for_character(&word_cube.despawn_character),
                character: word_cube.despawn_character,
            });

            for blessing in &mut current_letter.blessings {
                if blessing.target_word == prompt.text {
                    blessing.collected = true;
                }
            }

            for curse in &mut current_letter.curses {
                if curse.target_word == prompt.text {
                    curse.collected = true;
                }
            }
        }
    }

    prompt.text = "".into();

    if current_letter
        .blessings
        .iter()
        .all(|blessing| blessing.collected)
    {
        commands.trigger(LetterCleared);
    }
}

fn color_for_character(character: &char) -> ratatui::style::Color {
    match character {
        '+' => MAC_GREEN_COLOR,
        'x' => MAC_RED_COLOR,
        '~' => MAC_YELLOW_COLOR,
        _ => MAC_PURPLE_COLOR,
    }
}
