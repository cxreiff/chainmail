use std::ops::DerefMut;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::RatatuiContext;
use bevy_ratatui_camera::{RatatuiCameraDepthBuffer, RatatuiCameraWidget};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, StatefulWidget},
};

use super::{
    layout::layout_frame,
    widgets::current_letter::{CurrentLetter, CurrentLetterState},
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
    camera_widget: Single<&mut RatatuiCameraWidget>,
) -> Result {
    ratatui.draw(|frame| {
        let show_log_panel = !cfg!(feature = "windowed");
        let area = layout_frame(frame, &flags, &diagnostics, show_log_panel);

        let buffer = frame.buffer_mut();

        let left_width = (area.width * 2 / 5).min(120);
        let [left_area, right_area] =
            *Layout::horizontal([Constraint::Max(left_width), Constraint::Fill(1)])
                .split(area)
        else {
            unreachable!()
        };

        let letter_area = Block::new().inner(left_area);

        if let Some(current_letter) = current_letter {
            current_letter.render(letter_area, buffer, &mut current_letter_state);
        };

        let depth = &mut RatatuiCameraDepthBuffer::new(right_area);

        frame.render_stateful_widget(camera_widget.into_inner().deref_mut(), right_area, depth);
    })?;

    Ok(())
}
