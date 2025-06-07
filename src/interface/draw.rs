use std::ops::DerefMut;

use bevy::{diagnostic::DiagnosticsStore, prelude::*};
use bevy_ratatui::RatatuiContext;
use bevy_ratatui_camera::RatatuiCameraWidget;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Padding, Paragraph, StatefulWidget, Widget, Wrap},
};
use tachyonfx::{Effect, Interpolation, Shader, fx};

use crate::{
    constants::{
        MAC_PURPLE_COLOR, MAC_RED_COLOR, MAC_YELLOW_COLOR, PLASTIC_MEDIUM_BACKGROUND_COLOR,
    },
    letters::CurrentLetter,
    scene::spawning::WordCube,
    states::{GameStates, Statistics},
};

use super::{
    layout::layout_frame,
    widgets::{
        confetti::{Confetti, ConfettiWidget},
        letter::{LetterWidget, LetterWidgetState},
        prompt::{Prompt, PromptState},
    },
};

pub fn plugin(app: &mut App) {
    app.init_resource::<Flags>()
        .insert_non_send_resource(ResetEffect::default())
        .add_systems(Update, draw_system)
        .add_systems(OnEnter(GameStates::Resetting), activate_reset_scene_effect)
        .add_systems(OnExit(GameStates::Printing), deactivate_reset_scene_effect);
}

#[derive(Resource)]
pub struct Flags {
    pub debug: bool,
    pub sound: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            debug: false,
            sound: true,
        }
    }
}

fn draw_system(
    mut ratatui: ResMut<RatatuiContext>,
    flags: Res<Flags>,
    state: Res<State<GameStates>>,
    diagnostics: Res<DiagnosticsStore>,
    current_letter: Option<Res<CurrentLetter>>,
    mut current_letter_state: NonSendMut<LetterWidgetState>,
    camera: Single<(&Camera, &GlobalTransform, &mut RatatuiCameraWidget)>,
    stars: Query<(&WordCube, &Transform)>,
    prompt: Res<Prompt>,
    mut prompt_state: ResMut<PromptState>,
    confettis: Query<&Confetti>,
    reset_effect: NonSendMut<ResetEffect>,
    stats: Res<Statistics>,
    time: Res<Time>,
) -> Result {
    let (camera, camera_transform, camera_widget) = camera.into_inner();
    let mut camera_widget = camera_widget.into_inner();

    ratatui.draw(|frame| {
        let show_log_panel = !cfg!(feature = "windowed");
        let area = layout_frame(frame, &flags, &diagnostics, &stats, show_log_panel);

        let buf = frame.buffer_mut();

        if *state == GameStates::Info {
            let outer_block = Block::default().padding(Padding::proportional(2));
            let info_block = Block::bordered()
                .border_type(BorderType::Double)
                .padding(Padding::proportional(2))
                .bg(PLASTIC_MEDIUM_BACKGROUND_COLOR);

            let info_paragraph = Paragraph::new(Text::from(vec![
                Line::from("HOW TO PLAY").bold().fg(MAC_PURPLE_COLOR),
                Line::from(""),
                Line::from(
                    "Each round, a chain letter will appear on the left. Each letter has a list of \
                    blessings, for if the letter is forwarded, and a list of curses, for if the \
                    chain is broken. Each blessing and curse has a missing word. Your job is to \
                    look at the pool of words moving past on the right side of the screen, and \
                    figure out which ones correspond with the blessings.",
                ),
                Line::from(""),
                Line::from(
                    "Type your word and press enter. Matching blessings bestow money or score, \
                    curses take it away, and decoys do nothing (currently). Collect all the \
                    blessings to collect your income and move to the next round.",
                ),
                Line::from(""),
                Line::from("Someday the money will do something. Today is not that day."),
                Line::from(""),
                Line::from("Press TAB to toggle sound.").fg(MAC_RED_COLOR),
                Line::from(""),
                Line::from("PRESS SPACE TO BEGIN")
                    .bold()
                    .fg(MAC_PURPLE_COLOR),
            ]))
            .wrap(Wrap { trim: true });

            let inner_area = outer_block.inner(area);
            let inner_inner_area = info_block.inner(inner_area);
            info_block.render(inner_area, buf);
            info_paragraph.render(inner_inner_area, buf);

            return;
        }

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
            let star_line = if star.word.starts_with(&prompt.text) {
                Line::from(vec![
                    Span::from(&prompt.text).fg(MAC_YELLOW_COLOR).bold(),
                    Span::from(star.word.strip_prefix(&prompt.text).unwrap()).fg(lighter_color),
                ])
            } else {
                Line::from(star.word.clone()).fg(lighter_color)
            };
            star_widgets.push((
                star_line.bg(darker_color),
                Rect::new(
                    position.x as u16 - (star.word.len() as u16 / 2) + 1,
                    position.y as u16,
                    star.word.len() as u16,
                    1,
                ),
            ));
        }

        let mut confetti_widgets = vec![];
        for confetti in &confettis {
            let Some(ndc_coords) = camera.world_to_ndc(camera_transform, confetti.position) else {
                continue;
            };

            let cell = camera_widget.ndc_to_cell(scene_area, ndc_coords);

            confetti_widgets.push(ConfettiWidget::new(confetti, cell));
        }

        if let Some(current_letter) = current_letter {
            LetterWidget(&current_letter.0).render(left_area, buf, &mut current_letter_state);

            let character_pool: Vec<_> = current_letter
                .interpolated_flavor
                .body
                .chars()
                .chain([' '].into_iter())
                .chain(current_letter.interpolated_flavor.signoff.chars())
                .chain([' '].into_iter())
                .chain(current_letter.flavor.footer.chars())
                .chain([' '].into_iter())
                .collect();

            Widget::render(camera_widget.deref_mut(), scene_area, buf);

            for confetti_widget in &confetti_widgets {
                camera_widget.render_overlay(scene_area, buf, confetti_widget);
            }

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

        if let Some(ref mut reset_effect) = reset_effect.into_inner().0 {
            reset_effect.process(time.delta().into(), buf, scene_area);
        }

        prompt.render(prompt_area, buf, &mut prompt_state);
    })?;

    Ok(())
}

#[derive(Deref, DerefMut, Default)]
pub struct ResetEffect(pub Option<Effect>);

fn activate_reset_scene_effect(mut reset_effect: NonSendMut<ResetEffect>) {
    #[cfg(not(feature = "windowed"))]
    {
        reset_effect.0 = Some(Effect::new(fx::dissolve((1000, Interpolation::Linear))));
    }

    #[cfg(feature = "windowed")]
    {
        use crate::constants::PLASTIC_LIGHT_BACKGROUND_COLOR;
        use tachyonfx::Motion;

        reset_effect.0 = Some(Effect::new(fx::sweep_out(
            Motion::UpToDown,
            10,
            2,
            PLASTIC_LIGHT_BACKGROUND_COLOR,
            (1000, Interpolation::Linear),
        )));
    }
}

fn deactivate_reset_scene_effect(mut reset_effect: NonSendMut<ResetEffect>) {
    reset_effect.0 = None;
}
