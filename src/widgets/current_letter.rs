use std::time::Duration;

use bevy::prelude::*;
use ratatui::layout::{Alignment, Size};
use ratatui::style::{Color as RatatuiColor, Stylize};
use ratatui::widgets::{Block, BorderType, Padding, Wrap};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Paragraph, StatefulWidget, Widget},
};
use tachyonfx::{Effect, Interpolation, Motion, Shader, fx};
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::color_scheme::{
    MAC_GREEN_COLOR, MAC_RED_COLOR, PLASTIC_BACKGROUND_COLOR, PLASTIC_PRIMARY_COLOR,
    PLASTIC_SECONDARY_COLOR,
};
use crate::sound::SoundEffect;
use crate::{
    letters::{Letter, LetterAssets},
    states::GameStates,
};

// Reveal timing constants (in milliseconds)
const TITLE_REVEAL_TIME: u32 = 500;
const BODY_REVEAL_TIME: u32 = 5;
const HEADER_REVEAL_TIME: u32 = 400;
const BLESSING_REVEAL_TIME: u32 = 400;
const CURSE_REVEAL_TIME: u32 = 400;
const SIGNOFF_REVEAL_TIME: u32 = 5;
const FOOTER_REVEAL_TIME: u32 = 600;

// Margin of delay after each reveal section (in milliseconds).
const REVEAL_TIME_MARGIN: u32 = 400;

pub(super) fn plugin(app: &mut App) {
    app.insert_non_send_resource(CurrentLetterState::default())
        .add_systems(OnEnter(GameStates::Playing), letter_select_system)
        .add_systems(
            Update,
            letter_reveal_system.run_if(in_state(GameStates::Playing)),
        );
}

#[derive(Resource, Deref, DerefMut, Debug)]
pub struct CurrentLetter(pub Letter);

pub struct CurrentLetterState {
    effect: Effect,
    scroll_state: ScrollViewState,
    revealed: CurrentLetterRevealed,
}

impl Default for CurrentLetterState {
    fn default() -> Self {
        Self {
            effect: fx::slide_in(
                Motion::UpToDown,
                10,
                0,
                RatatuiColor::Rgb(0, 0, 0),
                (1000, Interpolation::Linear),
            ),
            scroll_state: ScrollViewState::default(),
            revealed: CurrentLetterRevealed::default(),
        }
    }
}

#[derive(Default)]
struct CurrentLetterRevealed {
    elapsed_ms: u32,
    title_revealed: bool,
    body_chars_revealed: usize,
    blessings_header_revealed: bool,
    blessings_revealed: usize,
    curses_header_revealed: bool,
    curses_revealed: usize,
    signoff_chars_revealed: usize,
    footer_revealed: bool,
}

impl CurrentLetterRevealed {
    pub fn next_state(&self, delta: Duration, letter: &Letter) -> CurrentLetterRevealed {
        let mut time_cursor = 0;
        let elapsed_ms = self.elapsed_ms + delta.as_millis() as u32;

        // title
        let title_revealed = elapsed_ms >= time_cursor + TITLE_REVEAL_TIME;
        time_cursor += TITLE_REVEAL_TIME + REVEAL_TIME_MARGIN;

        // body
        let body_time = letter.body.len() as u32 * BODY_REVEAL_TIME;
        let body_chars_revealed = if elapsed_ms > time_cursor {
            ((elapsed_ms - time_cursor) / BODY_REVEAL_TIME) as usize
        } else {
            0
        }.min(letter.body.len());
        time_cursor += body_time + REVEAL_TIME_MARGIN;

        // blessings header
        let blessings_header_revealed = elapsed_ms >= time_cursor + HEADER_REVEAL_TIME;
        time_cursor += HEADER_REVEAL_TIME;

        // blessings
        let blessings_revealed = if elapsed_ms > time_cursor {
            ((elapsed_ms - time_cursor) / BLESSING_REVEAL_TIME) as usize
        } else {
            0
        }.min(letter.blessings.len());
        time_cursor += letter.blessings.len() as u32 * BLESSING_REVEAL_TIME + REVEAL_TIME_MARGIN;

        // curses header
        let curses_header_revealed = elapsed_ms >= time_cursor + HEADER_REVEAL_TIME;
        time_cursor += HEADER_REVEAL_TIME;

        // curses
        let curses_revealed = if elapsed_ms > time_cursor {
            ((elapsed_ms - time_cursor) / CURSE_REVEAL_TIME) as usize
        } else {
            0
        }.min(letter.curses.len());
        time_cursor += letter.curses.len() as u32 * CURSE_REVEAL_TIME + REVEAL_TIME_MARGIN;

        // signoff
        let signoff_chars_revealed = if elapsed_ms > time_cursor {
            ((elapsed_ms - time_cursor) / SIGNOFF_REVEAL_TIME) as usize
        } else {
            0
        }.min(letter.signoff.len());
        time_cursor += letter.signoff.len() as u32 * SIGNOFF_REVEAL_TIME + REVEAL_TIME_MARGIN;

        // footer
        let footer_revealed = elapsed_ms >= time_cursor + FOOTER_REVEAL_TIME;

        CurrentLetterRevealed {
            elapsed_ms,
            title_revealed,
            body_chars_revealed,
            blessings_header_revealed,
            blessings_revealed,
            curses_header_revealed,
            curses_revealed,
            signoff_chars_revealed,
            footer_revealed,
        }
    }
}

impl StatefulWidget for &CurrentLetter {
    type State = CurrentLetterState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
            let mut lines: Vec<Line> = Vec::new();
            let CurrentLetterRevealed {
                title_revealed,
                body_chars_revealed,
                blessings_header_revealed,
                blessings_revealed,
                curses_header_revealed,
                curses_revealed,
                signoff_chars_revealed,
                footer_revealed,
                ..
            } = state.revealed;

            // title
            if title_revealed {
                lines.push(
                    Line::from(Span::styled(&self.title, Style::default().bold())).centered(),
                );
            }

            // body
            if body_chars_revealed > 0 {
                lines.push(Line::from(""));
                let revealed_body = &self.body[..body_chars_revealed.min(self.body.len())];
                for line in revealed_body.lines() {
                    lines.push(Line::from(line));
                }
                if body_chars_revealed >= self.body.len() {
                    lines.push(Line::from(""));
                }
            }

            // blessings header
            if blessings_header_revealed {
                lines.push(Line::from(Span::styled(
                    "THOSE THAT CONTINUED THE CHAIN:",
                    Style::default().italic(),
                )));
            }

            // blessings
            for i in 0..blessings_revealed.min(self.blessings.len()) {
                lines.push(Line::from(vec![
                    Span::styled("+ ", Style::default().fg(MAC_GREEN_COLOR)),
                    Span::raw(&self.blessings[i]),
                ]));
            }

            // curses header
            if curses_header_revealed {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "THOSE BROKE THE CHAIN:",
                    Style::default().italic(),
                )));
            }

            // curses
            for i in 0..curses_revealed.min(self.curses.len()) {
                lines.push(Line::from(vec![
                    Span::styled("- ", Style::default().fg(MAC_RED_COLOR)),
                    Span::raw(&self.curses[i]),
                ]));
            }

            // signoff
            if signoff_chars_revealed > 0 {
                lines.push(Line::from(""));
                let revealed_signoff =
                    &self.signoff[..signoff_chars_revealed.min(self.signoff.len())];
                lines.push(Line::from(revealed_signoff));
            }

            // footer
            if footer_revealed {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    &self.footer,
                    Style::default().fg(PLASTIC_SECONDARY_COLOR),
                )));
            }

            // wrap in paragraph
            let paragraph = Paragraph::new(Text::from(lines))
                .wrap(Wrap { trim: true })
                .fg(PLASTIC_PRIMARY_COLOR);

            // containing block
            let mut block = Block::bordered()
                .border_type(BorderType::QuadrantOutside)
                .border_style(Style::default().fg(PLASTIC_PRIMARY_COLOR))
                .padding(Padding::new(2, 2, 1, 0))
                .bg(PLASTIC_BACKGROUND_COLOR);
            let block_inner_area = block.inner(area);

            let paragraph_height = paragraph.line_count(block_inner_area.width);

            let scroll_buffer_size = Size::new(
                block_inner_area.width,
                (paragraph_height as u16).max(block_inner_area.height),
            );
            let scroll_area = Rect::new(0, 0, scroll_buffer_size.width, scroll_buffer_size.height);

            let mut scroll_view = ScrollView::new(scroll_buffer_size)
                .scrollbars_visibility(ScrollbarVisibility::Never);

            let hidden_rows = scroll_area.height.saturating_sub(block_inner_area.height);
            if state.scroll_state.offset().y < hidden_rows {
                block = block
                    .title_bottom(" ↓ SCROLL ↓ ")
                    .title_alignment(Alignment::Center);
            }
            if hidden_rows > 0 && state.scroll_state.offset().y > 0 {
                block = block
                    .title_top(" ↑ SCROLL ↑ ")
                    .title_alignment(Alignment::Center);
            }

            // draw
            scroll_view.render_widget(paragraph, scroll_area);
            scroll_view.render(block_inner_area, buf, &mut state.scroll_state);
            block.render(area, buf);

            // slide in effect
            state.effect.execute(Duration::ZERO.into(), area, buf);
    }
}

pub fn letter_select_system(
    mut commands: Commands,
    handles: Res<LetterAssets>,
    letters: Res<Assets<Letter>>,
) {
    commands.insert_resource(CurrentLetter(
        letters
            .get(handles.letters.first().unwrap())
            .cloned()
            .unwrap(),
    ));
}

pub fn letter_reveal_system(
    mut commands: Commands,
    current_letter: Option<Res<CurrentLetter>>,
    mut current_letter_state: NonSendMut<CurrentLetterState>,
    time: Res<Time>,
) {
    if let Some(ref current_letter) = current_letter {
        let new_revealed_state = current_letter_state
            .revealed
            .next_state(time.delta(), current_letter);

        trigger_reveal_sound_effects(commands.reborrow(), &current_letter_state.revealed, &new_revealed_state);

        current_letter_state.revealed = new_revealed_state;

        current_letter_state
            .effect
            .timer_mut()
            .and_then(|t| t.process(time.delta().into()));
    }
}

fn trigger_reveal_sound_effects(mut commands: Commands, previous: &CurrentLetterRevealed, next: &CurrentLetterRevealed) {
    if previous.title_revealed != next.title_revealed {
        commands.trigger(SoundEffect::Slam);
    }

    if previous.body_chars_revealed != next.body_chars_revealed {
        commands.trigger(SoundEffect::Tap);
    }

    if previous.blessings_header_revealed != next.blessings_header_revealed {
        commands.trigger(SoundEffect::BlessHeader);
    }

    if previous.blessings_revealed != next.blessings_revealed {
        commands.trigger(SoundEffect::Bless);
    }

    if previous.curses_header_revealed != next.curses_header_revealed {
        commands.trigger(SoundEffect::CurseHeader);
    }

    if previous.curses_revealed != next.curses_revealed {
        commands.trigger(SoundEffect::Curse);
    }

    if previous.signoff_chars_revealed != next.signoff_chars_revealed {
        commands.trigger(SoundEffect::Tap);
    }

    if previous.footer_revealed != next.footer_revealed {
        commands.trigger(SoundEffect::Slam);
    }
}
