use bevy::prelude::*;
use rand::{
    SeedableRng,
    seq::{IteratorRandom, SliceRandom},
};
use rand_chacha::ChaCha8Rng;
use tachyonfx::Shader;

use crate::{
    constants::{BLESSING_RANGE, CURSE_RANGE, DECOY_RANGE},
    interface::widgets::{letter::LetterWidgetState, prompt::Prompt},
    letters::{CurrentLetter, Flavor, LetterAssets, LetterBag, Name, TestimonialStub, WordBag},
    scene::spawning::WordCube,
    word_checks::WordGuesses,
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameStates>()
        .add_observer(letter_cleared_observer)
        .add_observer(letter_failed_observer)
        .add_systems(
            OnEnter(GameStates::Printing),
            generate_current_letter_system,
        )
        .add_systems(
            Update,
            resetting_timer_system.run_if(in_state(GameStates::Resetting)),
        )
        .add_systems(OnEnter(GameStates::Resetting), clear_prompt_system)
        .add_systems(OnExit(GameStates::Resetting), clear_word_cubes_system);
}

#[derive(Default, States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameStates {
    #[default]
    Loading,
    Info,
    Printing,
    Playing,
    Resetting,
}

#[derive(Event, Debug)]
pub struct LetterCleared;

#[derive(Event, Debug)]
pub struct LetterFailed;

#[derive(Resource, Deref, DerefMut)]
pub struct Rng(ChaCha8Rng);

impl Default for Rng {
    fn default() -> Self {
        Self(ChaCha8Rng::from_entropy())
    }
}

pub fn generate_current_letter_system(
    mut commands: Commands,
    mut letter_bag: ResMut<LetterBag>,
    letter_assets: Res<LetterAssets>,
    testimonials: Res<Assets<TestimonialStub>>,
    names: Res<Assets<Name>>,
    flavors: Res<Assets<Flavor>>,
    mut letter_widget_state: NonSendMut<LetterWidgetState>,
    mut rng: Local<Rng>,
) {
    let blessing_amount = BLESSING_RANGE
        .choose(&mut rng.0)
        .expect("min blessing amount should not be higher than max");
    let curse_amount = CURSE_RANGE
        .choose(&mut rng.0)
        .expect("min curse amount should not be higher than max");
    let decoy_amount = DECOY_RANGE
        .choose(&mut rng.0)
        .expect("min decoy amount should not be higher than max");

    let decoys: Vec<_> = (0..decoy_amount)
        .map(|_| {
            let decoy_handle = letter_assets
                .decoys
                .choose(&mut rng.0)
                .expect("decoys list should not be empty")
                .clone();

            let decoy = testimonials.get(&decoy_handle).unwrap();

            decoy
                .message
                .split_whitespace()
                .nth(decoy.targets[0])
                .expect("target word index must be valid")
        })
        .collect();

    let letter = letter_bag.pull_letter(
        &testimonials,
        flavors,
        names,
        &mut rng.0,
        blessing_amount,
        curse_amount,
    );

    commands.insert_resource(WordGuesses(letter.recipients));
    commands.insert_resource(WordBag::new(
        &letter.blessings,
        &letter.curses,
        &decoys,
        &mut rng.0,
    ));
    commands.insert_resource(CurrentLetter(letter));
    *letter_widget_state = LetterWidgetState::default();
}

pub fn letter_cleared_observer(_trigger: Trigger<LetterCleared>, mut commands: Commands) {
    commands.set_state(GameStates::Resetting);
}

pub fn letter_failed_observer(_trigger: Trigger<LetterFailed>, mut commands: Commands) {
    commands.set_state(GameStates::Resetting);
}

pub fn resetting_timer_system(
    mut commands: Commands,
    mut current_letter_state: NonSendMut<LetterWidgetState>,
) {
    if let Some(timer) = current_letter_state.effect.timer_mut() {
        if timer.done() {
            commands.set_state(GameStates::Printing);
        }
    }
}

fn clear_prompt_system(mut prompt: ResMut<Prompt>) {
    prompt.text = "".into();
}

fn clear_word_cubes_system(mut commands: Commands, word_cubes: Query<Entity, With<WordCube>>) {
    for entity in &word_cubes {
        commands.entity(entity).despawn();
    }
}
