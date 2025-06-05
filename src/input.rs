use crate::interface::widgets::prompt::Prompt;
#[cfg(feature = "windowed")]
use bevy::input::ButtonState;
#[cfg(feature = "windowed")]
use bevy::input::keyboard::KeyboardInput;
#[cfg(feature = "windowed")]
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
#[cfg(not(feature = "windowed"))]
use bevy_ratatui::event::KeyEvent as RatatuiKeyEvent;
#[cfg(not(feature = "windowed"))]
use bevy_ratatui::event::MouseEvent as RatatuiMouseEvent;

use crate::interface::draw::Flags;
use crate::interface::widgets::letter::LetterWidgetState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_keyboard_input_system,
            handle_mouse_input_system,
            handle_prompt_input_system,
        ),
    );
}

#[cfg(not(feature = "windowed"))]
fn handle_keyboard_input_system(
    mut commands: Commands,
    mut keyboard_input: EventReader<RatatuiKeyEvent>,
    mut flags: ResMut<Flags>,
    mut current_letter_state: NonSendMut<LetterWidgetState>,
) {
    use bevy_ratatui::crossterm::event::KeyCode;
    use bevy_ratatui::crossterm::event::KeyEventKind;

    use crate::word_checks::SubmittedWord;

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
            if let KeyCode::Enter = event.code {
                commands.trigger(SubmittedWord);
            }
        }
    }
}

#[cfg(not(feature = "windowed"))]
fn handle_mouse_input_system(
    mut mouse_input: EventReader<RatatuiMouseEvent>,
    mut letter_state: NonSendMut<LetterWidgetState>,
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
    mut current_letter_state: NonSendMut<LetterWidgetState>,
) {
    for &press in keyboard_input.get_just_pressed() {
        if press == KeyCode::Digit1 {
            flags.debug = !flags.debug;
        };
        if press == KeyCode::Digit2 {
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
    mut letter_state: NonSendMut<LetterWidgetState>,
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

#[cfg(not(feature = "windowed"))]
fn handle_prompt_input_system(
    mut keyboard_input: EventReader<RatatuiKeyEvent>,
    mut prompt_state: ResMut<Prompt>,
) {
    use bevy_ratatui::crossterm::event::KeyCode;
    use bevy_ratatui::crossterm::event::KeyEventKind;

    for event in keyboard_input.read() {
        if event.kind == KeyEventKind::Press {
            if let KeyCode::Char(c) = event.code {
                if c.is_alphabetic() {
                    prompt_state.text.push(c.to_ascii_lowercase());
                }
            } else if let KeyCode::Backspace = event.code {
                prompt_state.text.pop();
            }
        }
    }
}

#[cfg(feature = "windowed")]
fn handle_prompt_input_system(
    mut keyboard_events: EventReader<KeyboardInput>,
    mut prompt_state: ResMut<Prompt>,
) {
    for event in keyboard_events.read() {
        if event.state == ButtonState::Pressed {
            if let Some(text) = &event.text {
                for c in text.chars() {
                    if c.is_alphabetic() {
                        prompt_state.text.push(c.to_ascii_lowercase());
                    }
                }
            } else if event.key_code == KeyCode::Backspace {
                prompt_state.text.pop();
            }
        }
    }
}
