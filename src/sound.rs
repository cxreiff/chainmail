use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::{SeedableRng, seq::SliceRandom};
use rand_chacha::ChaCha8Rng;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SoundEffectRng>()
        .add_observer(sound_effects_observer);
}

#[derive(AssetCollection, Resource)]
pub struct SoundEffectAssets {
    #[asset(key = "sounds.slam", collection(typed))]
    pub slam: Vec<Handle<AudioSource>>,
    #[asset(key = "sounds.tap", collection(typed))]
    pub tap: Vec<Handle<AudioSource>>,
    #[asset(key = "sounds.bless_header")]
    pub bless_header: Handle<AudioSource>,
    #[asset(key = "sounds.bless", collection(typed))]
    pub bless: Vec<Handle<AudioSource>>,
    #[asset(key = "sounds.curse_header")]
    pub curse_header: Handle<AudioSource>,
    #[asset(key = "sounds.curse", collection(typed))]
    pub curse: Vec<Handle<AudioSource>>,
}

#[derive(Event)]
pub enum SoundEffect {
    Slam,
    Tap,
    BlessHeader,
    Bless,
    CurseHeader,
    Curse,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SoundEffectRng(ChaCha8Rng);

impl Default for SoundEffectRng {
    fn default() -> Self {
        Self(ChaCha8Rng::seed_from_u64(19878367467712))
    }
}

fn sound_effects_observer(
    trigger: Trigger<SoundEffect>,
    mut commands: Commands,
    handles: Res<SoundEffectAssets>,
    mut rng: ResMut<SoundEffectRng>,
) {
    let sound = match trigger.event() {
        SoundEffect::Slam => handles.slam.choose(&mut rng.0).unwrap(),
        SoundEffect::Tap => handles.tap.choose(&mut rng.0).unwrap(),
        SoundEffect::BlessHeader => &handles.bless_header,
        SoundEffect::Bless => handles.bless.choose(&mut rng.0).unwrap(),
        SoundEffect::CurseHeader => &handles.curse_header,
        SoundEffect::Curse => handles.curse.choose(&mut rng.0).unwrap(),
    };

    commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
}
