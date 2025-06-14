use crate::interface::widgets::prompt::Prompt;
use crate::states::GameStates;
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
use crate::sound::SoundEffect;
use crate::word_checks::SubmittedWord;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_keyboard_input_system,
            handle_mouse_input_system,
            handle_prompt_input_system,
            pass_info_screen_system.run_if(in_state(GameStates::Info)),
        ),
    );
}

#[cfg(not(feature = "windowed"))]
fn pass_info_screen_system(
    mut commands: Commands,
    mut keyboard_input: EventReader<RatatuiKeyEvent>,
) {
    use bevy_ratatui::crossterm::event::KeyCode;

    for event in keyboard_input.read() {
        if event.code == KeyCode::Char(' ') {
            commands.set_state(GameStates::Printing);
        }
    }
}

#[cfg(feature = "windowed")]
fn pass_info_screen_system(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    for &press in keyboard_input.get_just_pressed() {
        if press == KeyCode::Space {
            commands.set_state(GameStates::Printing);
        };
    }
}

#[cfg(not(feature = "windowed"))]
fn handle_keyboard_input_system(
    mut commands: Commands,
    mut keyboard_input: EventReader<RatatuiKeyEvent>,
    mut flags: ResMut<Flags>,
    mut current_letter_state: NonSendMut<LetterWidgetState>,
    game_state: Res<State<GameStates>>,
) {
    use bevy_ratatui::crossterm::event::KeyCode;
    use bevy_ratatui::crossterm::event::KeyEventKind;

    for event in keyboard_input.read() {
        if event.kind == KeyEventKind::Press {
            if event.code == KeyCode::Char('=') {
                flags.debug = !flags.debug;
            }
            if event.code == KeyCode::Tab {
                flags.sound = !flags.sound;
            }
            if event.code == KeyCode::Up {
                current_letter_state.scroll_state.scroll_up();
            }
            if event.code == KeyCode::Down {
                current_letter_state.scroll_state.scroll_down();
            }
            if event.code == KeyCode::Enter && *game_state == GameStates::Playing {
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
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut flags: ResMut<Flags>,
    mut current_letter_state: NonSendMut<LetterWidgetState>,
    game_state: Res<State<GameStates>>,
) {
    for &press in keyboard_input.get_just_pressed() {
        if press == KeyCode::Tab {
            flags.sound = !flags.sound;
        };
        if press == KeyCode::ArrowUp {
            current_letter_state.scroll_state.scroll_up();
        }
        if press == KeyCode::ArrowDown {
            current_letter_state.scroll_state.scroll_down();
        }
        if press == KeyCode::Enter && *game_state == GameStates::Playing {
            commands.trigger(SubmittedWord);
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
    mut commands: Commands,
    mut keyboard_input: EventReader<RatatuiKeyEvent>,
    mut prompt_state: ResMut<Prompt>,
) {
    use bevy_ratatui::crossterm::event::KeyCode;
    use bevy_ratatui::crossterm::event::KeyEventKind;

    for event in keyboard_input.read() {
        if event.kind == KeyEventKind::Press {
            if let KeyCode::Char(c) = event.code {
                if c.is_alphabetic() {
                    commands.trigger(SoundEffect::TextCharacter);
                    prompt_state.text.push(c.to_ascii_lowercase());
                }
            } else if let KeyCode::Backspace = event.code {
                commands.trigger(SoundEffect::TextCharacter);
                prompt_state.text.pop();
            }
        }
    }
}

#[cfg(feature = "windowed")]
fn handle_prompt_input_system(
    mut commands: Commands,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut prompt_state: ResMut<Prompt>,
) {
    for event in keyboard_events.read() {
        if event.state == ButtonState::Pressed {
            if let Some(text) = &event.text {
                for c in text.chars() {
                    if c.is_alphabetic() {
                        commands.trigger(SoundEffect::TextCharacter);
                        prompt_state.text.push(c.to_ascii_lowercase());
                    }
                }
            } else if event.key_code == KeyCode::Backspace {
                commands.trigger(SoundEffect::TextCharacter);
                prompt_state.text.pop();
            }
        }
    }
}
