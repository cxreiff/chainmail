use std::ops::DerefMut;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::RatatuiContext;
use bevy_ratatui_camera::RatatuiCameraWidget;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Padding, StatefulWidget, Widget},
};

use crate::{letters::CurrentLetter, scene::spawning::Star};

use super::{
    layout::layout_frame,
    widgets::{
        letter::{LetterWidget, LetterWidgetState},
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
    mut current_letter_state: NonSendMut<LetterWidgetState>,
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
            let Srgba {
                red: lighter_red,
                blue: lighter_blue,
                green: lighter_green,
                ..
            } = star.color.lighter(0.3).to_srgba();
            let lighter_color = ratatui::style::Color::Rgb(
                (lighter_red * 256.) as u8,
                (lighter_green * 256.) as u8,
                (lighter_blue * 256.) as u8,
            );
            let Srgba {
                red: darker_red,
                blue: darker_blue,
                green: darker_green,
                ..
            } = star.color.darker(0.0).to_srgba();
            let darker_color = ratatui::style::Color::Rgb(
                (darker_red * 256.) as u8,
                (darker_green * 256.) as u8,
                (darker_blue * 256.) as u8,
            );
            star_widgets.push((
                Line::from(star.word.clone())
                    .fg(lighter_color)
                    .bg(darker_color),
                Rect::new(
                    position.x as u16,
                    position.y as u16,
                    star.word.len() as u16,
                    1,
                ),
            ));
        }

        if let Some(current_letter) = current_letter {
            LetterWidget(&current_letter.0).render(left_area, buf, &mut current_letter_state);

            let character_pool: Vec<_> = current_letter.flavor.body.chars().collect();

            Widget::render(camera_widget.into_inner().deref_mut(), scene_area, buf);

            for position in scene_area.positions() {
                if buf[position].symbol() == "@" {
                    let _offset = (prompt.timer.remaining_secs() * 14.0) as u16;
                    let index = position.x + position.y * scene_area.width;
                    let wrapped_index = index as usize % character_pool.len();
                    buf[position].set_char(character_pool[wrapped_index]);
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
        };

        prompt.render(prompt_area, buf, &mut prompt_state);
    })?;

    Ok(())
}
