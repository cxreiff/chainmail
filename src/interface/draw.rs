use std::ops::DerefMut;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::RatatuiContext;
use bevy_ratatui_camera::RatatuiCameraWidget;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, StatefulWidget, Widget},
};

use crate::scene::spawning::Star;

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
    camera: Single<(&Camera, &GlobalTransform, &mut RatatuiCameraWidget)>,
    stars: Query<(&Star, &Transform)>,
) -> Result {
    let (camera, camera_transform, camera_widget) = camera.into_inner();

    ratatui.draw(|frame| {
        let show_log_panel = !cfg!(feature = "windowed");
        let area = layout_frame(frame, &flags, &diagnostics, show_log_panel);

        let buffer = frame.buffer_mut();

        let left_width = (area.width * 2 / 5).min(120);
        let [left_area, right_area] =
            *Layout::horizontal([Constraint::Max(left_width), Constraint::Fill(1)]).split(area)
        else {
            unreachable!()
        };

        let letter_area = Block::new().inner(left_area);

        if let Some(current_letter) = current_letter {
            current_letter.render(letter_area, buffer, &mut current_letter_state);
        };

        let mut star_widgets = vec![];
        for (star, star_transform) in &stars {
            let Some(ndc_coords) =
                camera.world_to_ndc(camera_transform, star_transform.translation)
            else {
                continue;
            };

            let position = camera_widget.ndc_to_cell(right_area, ndc_coords);
            star_widgets.push((
                Line::from(star.word.clone()),
                Rect::new(
                    position.x as u16,
                    position.y as u16,
                    star.word.len() as u16,
                    1,
                ),
            ));
        }

        Widget::render(camera_widget.into_inner().deref_mut(), right_area, buffer);

        for (star_widget, star_area) in &star_widgets {
            star_widget.render(*star_area, buffer);
        }
    })?;

    Ok(())
}
