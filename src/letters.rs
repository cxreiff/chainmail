use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_shuffle_bag::ShuffleBag;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Deserialize;

use crate::states::GameStates;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Blessing>::new(&["blessing.ron"]))
        .add_plugins(RonAssetPlugin::<Curse>::new(&["curse.ron"]))
        .add_plugins(RonAssetPlugin::<Flavor>::new(&["flavor.ron"]))
        .add_systems(OnExit(GameStates::Loading), create_letter_bag_system);
}

#[derive(AssetCollection, Resource)]
pub struct LetterAssets {
    #[asset(key = "letters.blessings", collection(typed))]
    pub blessings: Vec<Handle<Blessing>>,
    #[asset(key = "letters.curses", collection(typed))]
    pub curses: Vec<Handle<Curse>>,
    #[asset(key = "letters.flavors", collection(typed))]
    pub flavors: Vec<Handle<Flavor>>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Effect {
    Score(i32),
    Money(i32),
    Income(i32),
}

#[derive(Debug, Deserialize, Asset, TypePath, Clone)]
pub struct Blessing {
    pub message: String,
    pub effect: Effect,
    pub targets: Vec<usize>,
}

#[derive(Debug, Deserialize, Asset, TypePath, Clone)]
pub struct Curse {
    pub message: String,
    pub effect: Effect,
    pub targets: Vec<usize>,
}

#[derive(Debug, Deserialize, Asset, TypePath, Clone)]
pub struct Flavor {
    pub title: String,
    pub body: String,
    pub signoff: String,
    pub footer: String,
}

#[derive(Debug, Clone)]
pub struct Letter {
    pub flavor: Flavor,
    pub blessings: Vec<Blessing>,
    pub curses: Vec<Curse>,
}

impl Letter {
    pub fn new(flavor: Flavor, blessings: Vec<Blessing>, curses: Vec<Curse>) -> Self {
        Self {
            flavor,
            blessings,
            curses,
        }
    }
}

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct CurrentLetter(pub Letter);

#[derive(Resource, Debug)]
pub struct LetterBag {
    pub flavors: ShuffleBag<Handle<Flavor>>,
    pub blessings: ShuffleBag<Handle<Blessing>>,
    pub curses: ShuffleBag<Handle<Curse>>,
}

impl LetterBag {
    pub fn pull_letter<R>(
        &mut self,
        blessings: Res<Assets<Blessing>>,
        curses: Res<Assets<Curse>>,
        flavors: Res<Assets<Flavor>>,
        rng: &mut R,
        blessing_amount: usize,
        curse_amount: usize,
    ) -> Letter
    where
        R: Rng,
    {
        let flavor_handle = self.flavors.pick(rng);
        let flavor = flavors
            .get(flavor_handle)
            .expect("flavor asset must be present")
            .to_owned();

        let blessings: Vec<Blessing> = (0..blessing_amount)
            .map(|_| self.blessings.pick(rng).clone())
            .map(|h| {
                blessings
                    .get(&h)
                    .expect("blessing asset must be present")
                    .to_owned()
            })
            .collect();
        let curses: Vec<_> = (0..curse_amount)
            .map(|_| self.curses.pick(rng).clone())
            .map(|h| {
                curses
                    .get(&h)
                    .expect("curse asset must be present")
                    .to_owned()
            })
            .collect();

        Letter {
            flavor,
            blessings,
            curses,
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct RngResource(ChaCha8Rng);

impl Default for RngResource {
    fn default() -> Self {
        Self(ChaCha8Rng::seed_from_u64(19878367867912))
    }
}

fn create_letter_bag_system(
    mut commands: Commands,
    letter_handles: Res<LetterAssets>,
    mut rng: Local<RngResource>,
) {
    commands.insert_resource(LetterBag {
        flavors: ShuffleBag::try_new(letter_handles.flavors.clone(), &mut rng.0)
            .expect("flavor handle list should not be empty"),
        blessings: ShuffleBag::try_new(letter_handles.blessings.clone(), &mut rng.0)
            .expect("blessing handle list should not be empty"),
        curses: ShuffleBag::try_new(letter_handles.curses.clone(), &mut rng.0)
            .expect("curse handle list should not be empty"),
    });
}
