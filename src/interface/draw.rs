use std::ops::DerefMut;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::RatatuiContext;
use bevy_ratatui_camera::RatatuiCameraWidget;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Padding, StatefulWidget, Widget},
};

use crate::scene::spawning::Star;

use super::{
    layout::layout_frame,
    widgets::{
        current_letter::{CurrentLetter, CurrentLetterState},
        prompt::{Prompt, PromptState},
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
    camera: Single<(&Camera, &GlobalTransform, &mut RatatuiCameraWidget)>,
    stars: Query<(&Star, &Transform)>,
    prompt: Res<Prompt>,
    mut prompt_state: ResMut<PromptState>,
) -> Result {
    let (camera, camera_transform, camera_widget) = camera.into_inner();

    ratatui.draw(|frame| {
        let show_log_panel = !cfg!(feature = "windowed");
        let area = layout_frame(frame, &flags, &diagnostics, show_log_panel);

        let buf = frame.buffer_mut();

        let left_width = (area.width * 2 / 5).min(120);
        let [left_area, right_area] =
            *Layout::horizontal([Constraint::Max(left_width), Constraint::Fill(1)]).split(area)
        else {
            unreachable!()
        };

        let [scene_area, prompt_area] =
            *Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]).split(right_area)
        else {
            unreachable!()
        };

        let prompt_area = Block::default()
            .padding(Padding::new(0, 2, 0, 1))
            .inner(prompt_area);

        let mut star_widgets = vec![];
        for (star, star_transform) in &stars {
            let Some(ndc_coords) =
                camera.world_to_ndc(camera_transform, star_transform.translation)
            else {
                continue;
            };

            let position = camera_widget.ndc_to_cell(scene_area, ndc_coords);
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

        let mut character_pool = vec![];
        if let Some(current_letter) = current_letter {
            current_letter.render(left_area, buf, &mut current_letter_state);

            character_pool = current_letter.body.chars().collect();
        };

        Widget::render(camera_widget.into_inner().deref_mut(), scene_area, buf);

        for position in scene_area.positions() {
            if buf[position].symbol() == "@" {
                let index =
                    (position.x + position.y * scene_area.width) as usize % character_pool.len();
                buf[position].set_char(character_pool[index]);
            }
        }

        for (index, cell) in buf.content.iter_mut().enumerate() {
            if cell.symbol() == "@" {
                let character = character_pool[index % character_pool.len()];
                cell.set_char(character);
            }
        }

        for (star_widget, star_area) in &star_widgets {
            if scene_area.contains((star_area.x, star_area.y).into())
                && scene_area.contains(
                    (
                        star_area.x + star_area.width,
                        star_area.y + star_area.height,
                    )
                        .into(),
                )
            {
                star_widget.render(*star_area, buf);
            }
        }

        prompt.render(prompt_area, buf, &mut prompt_state);
    })?;

    Ok(())
}
