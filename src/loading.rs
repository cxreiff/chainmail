use bevy::prelude::*;
use bevy_asset_loader::{
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
    standard_dynamic_asset::StandardDynamicAssetCollection,
};

use crate::{letters::LetterAssets, sound::SoundEffectAssets, states::GameStates};

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(GameStates::Loading)
            .continue_to_state(GameStates::Playing)
            .with_dynamic_assets_file::<StandardDynamicAssetCollection>("assets.ron")
            .load_collection::<LetterAssets>()
            .load_collection::<SoundEffectAssets>(),
    );
}
