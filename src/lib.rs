use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_ratatui::RatatuiPlugins;

mod constants;
mod draw;
mod input;
mod letters;
mod loading;
mod sound;
// mod scene;
mod states;
#[cfg(not(feature = "windowed"))]
mod terminal;
mod widgets;
#[cfg(feature = "windowed")]
mod windowed;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin {
                smoothing_factor: 1.0,
                ..default()
            },
            EntityCountDiagnosticsPlugin,
            RatatuiPlugins {
                enable_mouse_capture: true,
                ..default()
            },
        ));

        app.add_plugins((
            #[cfg(not(feature = "windowed"))]
            terminal::plugin,
            #[cfg(feature = "windowed")]
            windowed::plugin,
        ));

        app.add_plugins((
            draw::plugin,
            input::plugin,
            letters::plugin,
            loading::plugin,
            sound::plugin,
            // scene::plugin,
            states::plugin,
            widgets::plugin,
        ));

        app.insert_resource(ClearColor(Color::srgba(0., 0., 0., 0.)));
    }
}
