// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod draw;
mod input;
mod scene;

#[cfg(not(feature = "windowed"))]
use std::time::Duration;

#[cfg(not(feature = "windowed"))]
use bevy::app::ScheduleRunnerPlugin;
#[cfg(feature = "windowed")]
use bevy::asset::AssetMetaCheck;
#[cfg(not(feature = "windowed"))]
use bevy::log::LogPlugin;
#[cfg(not(feature = "windowed"))]
use bevy::winit::WinitPlugin;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_ratatui::RatatuiPlugins;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Bevy plugins for windowed mode.
        #[cfg(feature = "windowed")]
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Chainmail".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Bevy plugins for terminal mode.
        #[cfg(not(feature = "windowed"))]
        app.add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>(),
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(1. / 60.)),
        ));

        // Plugins common to both modes.
        app.insert_resource(ClearColor(Color::srgba(0., 0., 0., 0.)))
            .add_plugins((
                FrameTimeDiagnosticsPlugin {
                    smoothing_factor: 1.0,
                    ..default()
                },
                RatatuiPlugins {
                    enable_mouse_capture: true,
                    ..default()
                },
                draw::plugin,
                input::plugin,
                scene::plugin,
            ));
    }
}
