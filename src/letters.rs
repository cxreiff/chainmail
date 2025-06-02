use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::states::GameStates;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<Letter>::new(&["letter.ron"]))
        .add_systems(OnEnter(GameStates::Playing), letter_print_system);
}

#[derive(AssetCollection, Resource)]
pub struct LetterAssets {
    #[asset(key = "letters", collection(typed))]
    pub letters: Vec<Handle<Letter>>,
}

#[derive(Debug, Deserialize, Asset, TypePath, Clone)]
pub struct Letter {
    pub title: String,
    pub body: String,
    pub blessings: Vec<String>,
    pub curses: Vec<String>,
    pub signoff: String,
    pub footer: String,
}

fn letter_print_system(letters: Res<LetterAssets>) {
    log::info!("{:?}", letters.letters);
}
