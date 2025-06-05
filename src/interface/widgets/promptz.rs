use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Widget},
};

use crate::constants::{
    CURSOR_BLINK_SPEED, CUSTOM_BORDER_UNDER, PLASTIC_DARK_BACKGROUND_COLOR, PLASTIC_PRIMARY_COLOR,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PromptState>().add_systems(
        Update,
        (
            toggle_cursor_system.run_if(on_timer(Duration::from_millis(CURSOR_BLINK_SPEED))),
            update_timer_system,
        ),
    );
}

#[derive(Resource)]
pub struct PromptState {
    pub text: String,
    pub cursor_visible: bool,
    pub timer: Timer,
    pub total_time: Duration,
}

impl Default for PromptState {
    fn default() -> Self {
        let total_time = Duration::from_secs(60); // 1 minute default
        Self {
            text: String::new(),
            cursor_visible: false,
            timer: Timer::new(total_time, TimerMode::Once),
            total_time,
        }
    }
}

fn toggle_cursor_system(mut prompt_state: ResMut<PromptState>) {
    prompt_state.cursor_visible = !prompt_state.cursor_visible;
}

fn update_timer_system(mut prompt_state: ResMut<PromptState>, time: Res<Time>) {
    prompt_state.timer.tick(time.delta());
}

pub struct PromptWidget<'a> {
    prompt_text: &'a str,
    cursor_visible: bool,
    timer_progress: f64,
    remaining_seconds: u64,
}

impl<'a> PromptWidget<'a> {
    pub fn new(
        prompt_text: &'a str,
        cursor_visible: bool,
        timer_progress: f64,
        remaining_seconds: u64,
    ) -> Self {
        Self {
            prompt_text,
            cursor_visible,
            timer_progress,
            remaining_seconds,
        }
    }
}

impl Widget for PromptWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Add padding to the widget area
        let padded_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width.saturating_sub(2), // 2 chars padding on the right
            height: area.height.saturating_sub(1), // 1 char padding below
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(CUSTOM_BORDER_UNDER)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(padded_area);
        block.render(padded_area, buf);

        // Split the inner area: prompt on left, timer on right
        let [prompt_area, timer_area] = *Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(8), // Space for "XXXs" + some padding
        ])
        .split(inner) else {
            return;
        };

        // Render the prompt
        let cursor = if self.cursor_visible { "_" } else { " " };

        let prompt_spans = if self.prompt_text.is_empty() {
            vec![
                Span::styled(" ❯ ", Style::default().fg(PLASTIC_PRIMARY_COLOR)),
                Span::styled(cursor, Style::default().fg(PLASTIC_PRIMARY_COLOR)),
            ]
        } else {
            vec![
                Span::styled(
                    format!(" ❯ {} ", self.prompt_text),
                    Style::default().fg(PLASTIC_PRIMARY_COLOR),
                ),
                Span::styled(cursor, Style::default().fg(PLASTIC_PRIMARY_COLOR)),
            ]
        };
        let paragraph = Paragraph::new(Line::from(prompt_spans));
        paragraph.render(prompt_area, buf);

        // Render the timer gauge
        let gauge = Gauge::default()
            .ratio(self.timer_progress)
            .label(format!("{}s", self.remaining_seconds))
            .gauge_style(
                Style::default()
                    .fg(PLASTIC_PRIMARY_COLOR)
                    .bg(PLASTIC_DARK_BACKGROUND_COLOR),
            )
            .use_unicode(true);

        gauge.render(timer_area, buf);
    }
}

pub fn draw_prompt(area: Rect, frame: &mut Frame, prompt_state: &PromptState) {
    let progress = if prompt_state.timer.finished() {
        0.0
    } else {
        1.0 - (prompt_state.timer.elapsed().as_secs_f64() / prompt_state.total_time.as_secs_f64())
    };

    let remaining_seconds = if prompt_state.timer.finished() {
        0
    } else {
        (prompt_state.total_time.as_secs() as i64 - prompt_state.timer.elapsed().as_secs() as i64)
            .max(0) as u64
    };

    let widget = PromptWidget::new(
        &prompt_state.text,
        prompt_state.cursor_visible,
        progress,
        remaining_seconds,
    );
    frame.render_widget(widget, area);
}

