use bevy::prelude::*;

use crate::draw::Flags;
#[cfg(not(feature = "windowed"))]
use bevy_ratatui::event::KeyEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_input_system);
}

#[cfg(not(feature = "windowed"))]
pub fn handle_input_system(mut crossterm_input: EventReader<KeyEvent>, mut flags: ResMut<Flags>) {
    use bevy_ratatui::crossterm::event::KeyCode;
    use bevy_ratatui::crossterm::event::KeyEventKind;
    for event in crossterm_input.read() {
        if event.kind == KeyEventKind::Press {
            if let KeyCode::Char('d') = event.code {
                flags.debug = !flags.debug;
            }
        }
    }
}

#[cfg(feature = "windowed")]
pub fn handle_input_system(bevy_input: Res<ButtonInput<KeyCode>>, mut flags: ResMut<Flags>) {
    for &press in bevy_input.get_just_pressed() {
        if press == KeyCode::KeyD {
            flags.debug = !flags.debug;
        };
    }
}
