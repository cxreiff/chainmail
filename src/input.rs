#[cfg(feature = "windowed")]
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
#[cfg(not(feature = "windowed"))]
use bevy_ratatui::event::KeyEvent as RatatuiKeyEvent;
#[cfg(not(feature = "windowed"))]
use bevy_ratatui::event::MouseEvent as RatatuiMouseEvent;

use crate::interface::draw::Flags;
use crate::interface::widgets::current_letter::CurrentLetterState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (handle_keyboard_input_system, handle_mouse_input_system),
    );
}

#[cfg(not(feature = "windowed"))]
fn handle_keyboard_input_system(
    mut keyboard_input: EventReader<RatatuiKeyEvent>,
    mut flags: ResMut<Flags>,
    mut current_letter_state: NonSendMut<CurrentLetterState>,
) {
    use bevy_ratatui::crossterm::event::KeyCode;
    use bevy_ratatui::crossterm::event::KeyEventKind;

    for event in keyboard_input.read() {
        if event.kind == KeyEventKind::Press {
            if let KeyCode::Char('1') = event.code {
                flags.debug = !flags.debug;
            }
            if let KeyCode::Char('2') = event.code {
                flags.sound = !flags.sound;
            }
            if let KeyCode::Up = event.code {
                current_letter_state.scroll_state.scroll_up();
            }
            if let KeyCode::Down = event.code {
                current_letter_state.scroll_state.scroll_down();
            }
        }
    }
}

#[cfg(not(feature = "windowed"))]
fn handle_mouse_input_system(
    mut mouse_input: EventReader<RatatuiMouseEvent>,
    mut letter_state: NonSendMut<CurrentLetterState>,
) {
    use ratatui::crossterm::event::MouseEventKind;

    for event in mouse_input.read() {
        match event.kind {
            MouseEventKind::ScrollUp => {
                letter_state.scroll_state.scroll_up();
            }
            MouseEventKind::ScrollDown => {
                letter_state.scroll_state.scroll_down();
            }
            _ => {}
        }
    }
}

#[cfg(feature = "windowed")]
fn handle_keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut flags: ResMut<Flags>,
    mut current_letter_state: NonSendMut<CurrentLetterState>,
) {
    for &press in keyboard_input.get_just_pressed() {
        if press == KeyCode::KeyD {
            flags.debug = !flags.debug;
        };
        if press == KeyCode::KeyM {
            flags.sound = !flags.sound;
        };
        if press == KeyCode::ArrowUp {
            current_letter_state.scroll_state.scroll_up();
        }
        if press == KeyCode::ArrowDown {
            current_letter_state.scroll_state.scroll_down();
        }
    }
}

#[cfg(feature = "windowed")]
fn handle_mouse_input_system(
    mut letter_state: NonSendMut<CurrentLetterState>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_wheel_events.read() {
        if event.y > 0.0 {
            letter_state.scroll_state.scroll_up();
        } else if event.y < 0.0 {
            letter_state.scroll_state.scroll_down();
        }
    }
}
