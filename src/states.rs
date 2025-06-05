use bevy::prelude::*;
use rand::{SeedableRng, seq::IteratorRandom};
use rand_chacha::ChaCha8Rng;

use crate::{
    constants::{MAX_BLESSING_AMOUNT, MAX_CURSE_AMOUNT, MIN_BLESSING_AMOUNT, MIN_CURSE_AMOUNT},
    interface::widgets::letter::LetterWidgetState,
    letters::{Blessing, CurrentLetter, Curse, Flavor, LetterBag},
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameStates>().add_systems(
        OnEnter(GameStates::Printing),
        generate_current_letter_system,
    );
}

#[derive(Default, States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum GameStates {
    #[default]
    Loading,
    Printing,
    Playing,
    _Resetting,
}

#[derive(Resource, Deref, DerefMut)]
pub struct Rng(ChaCha8Rng);

impl Default for Rng {
    fn default() -> Self {
        Self(ChaCha8Rng::seed_from_u64(19874567867912))
    }
}

fn generate_current_letter_system(
    mut commands: Commands,
    mut letter_bag: ResMut<LetterBag>,
    blessings: Res<Assets<Blessing>>,
    curses: Res<Assets<Curse>>,
    flavors: Res<Assets<Flavor>>,
    mut rng: Local<Rng>,
    mut letter_widget_state: NonSendMut<LetterWidgetState>,
) {
    let blessing_amount = (MIN_BLESSING_AMOUNT..=MAX_BLESSING_AMOUNT)
        .choose(&mut rng.0)
        .expect("min blessing amount should not be higher than max");
    let curse_amount = (MIN_CURSE_AMOUNT..=MAX_CURSE_AMOUNT)
        .choose(&mut rng.0)
        .expect("min curse amount should not be higher than max");
    let letter = letter_bag.pull_letter(
        blessings,
        curses,
        flavors,
        &mut rng.0,
        blessing_amount,
        curse_amount,
    );

    commands.insert_resource(CurrentLetter(letter));
    *letter_widget_state = LetterWidgetState::default();
}
