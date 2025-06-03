use bevy::prelude::*;
use bevy_ratatui_camera::{
    ColorChoice, ColorsConfig, EdgeCharacters, LuminanceConfig, RatatuiCamera,
    RatatuiCameraEdgeDetection, RatatuiCameraStrategy,
};

use crate::states::GameStates;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameStates::Playing), camera_setup_system);
}

fn camera_setup_system(mut commands: Commands) {
    commands.spawn((
        RatatuiCamera::default(),
        RatatuiCameraStrategy::Luminance(LuminanceConfig {
            colors: ColorsConfig {
                background: Some(ColorChoice::Scale(0.2)),
                ..default()
            },
            ..default()
        }),
        RatatuiCameraEdgeDetection {
            edge_characters: EdgeCharacters::Single('+'),
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
    ));
}
