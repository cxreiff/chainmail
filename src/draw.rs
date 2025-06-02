use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::RatatuiContext;
use ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, BorderType, Padding, StatefulWidget, Widget},
};

use crate::{
    color_scheme::PLASTIC_PRIMARY_COLOR,
    widgets::{
        current_letter::{CurrentLetter, CurrentLetterState},
        debug_frame::debug_frame,
    },
};

pub fn plugin(app: &mut App) {
    app.init_resource::<Flags>()
        .add_systems(Update, draw_system);
}

#[derive(Resource, Default)]
pub struct Flags {
    pub debug: bool,
    pub sound: bool,
}

fn draw_system(
    mut ratatui: ResMut<RatatuiContext>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    current_letter: Option<Res<CurrentLetter>>,
    mut current_letter_state: NonSendMut<CurrentLetterState>,
) -> Result {
    ratatui.draw(|frame| {
        let show_log_panel = !cfg!(feature = "windowed");
        let area = debug_frame(frame, &flags, &diagnostics, show_log_panel);

        let buffer = frame.buffer_mut();

        let [left_area, right_area] =
            *Layout::horizontal(Constraint::from_fills([1, 1])).split(area)
        else {
            unreachable!()
        };

        let letter_area = Block::new()
            .padding(Padding::new(4, 2, 2, 2))
            .inner(left_area);

        if let Some(current_letter) = current_letter {
            current_letter.render(letter_area, buffer, &mut current_letter_state);
        };

        let game_area = Block::new()
            .padding(Padding::new(2, 4, 2, 2))
            .inner(right_area);

        let game_block = Block::bordered()
            .border_type(BorderType::QuadrantOutside)
            .border_style(Style::default().fg(PLASTIC_PRIMARY_COLOR));

        game_block.render(game_area, buffer);
    })?;

    Ok(())
}
