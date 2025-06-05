use bevy::prelude::*;
use bevy_ratatui_camera::{RatatuiCamera, RatatuiCameraStrategy};

use crate::states::GameStates;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameStates::Playing), camera_setup_system);
}

fn camera_setup_system(mut commands: Commands) {
    commands.spawn((
        Camera {
            order: 1,
            ..default()
        },
        RatatuiCamera::default(),
        RatatuiCameraStrategy::luminance_with_characters(&[' ', '@']),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Off,
    ));
}
