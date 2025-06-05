use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, StatefulWidget, Widget},
};

use crate::{
    constants::{
        CURSOR_BLINK_SPEED, CUSTOM_BORDERS_UNDER, MAC_PURPLE_MUTED_COLOR, PLASTIC_EMPHASIS_COLOR,
        PLASTIC_LIGHTER_BACKGROUND_COLOR, PLASTIC_MEDIUM_BACKGROUND_COLOR, PLASTIC_PRIMARY_COLOR,
        PLASTIC_SECONDARY_COLOR,
    },
    states::GameStates,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Prompt>()
        .init_resource::<PromptState>()
        .add_systems(OnEnter(GameStates::Playing), timer_setup_system)
        .add_systems(
            Update,
            (
                tick_timer_system,
                blink_prompt_system.run_if(on_timer(Duration::from_millis(CURSOR_BLINK_SPEED))),
            ),
        );
}

#[derive(Resource, Default)]
pub struct Prompt {
    pub text: String,
    pub timer: Timer,
}

#[derive(Resource, Default)]
pub struct PromptState {
    cursor_visible: bool,
}

impl StatefulWidget for &Prompt {
    type State = PromptState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .border_set(CUSTOM_BORDERS_UNDER)
            .fg(PLASTIC_PRIMARY_COLOR);

        let inner_area = block.inner(area);

        let [text_area, timer_area] =
            *Layout::horizontal([Constraint::Fill(1), Constraint::Length(7)]).split(inner_area)
        else {
            unreachable!()
        };

        let [remaining_area, elapsed_area] = *Layout::horizontal([
            Constraint::Percentage((self.timer.fraction_remaining() * 100.0) as u16),
            Constraint::Fill(1),
        ])
        .split(area) else {
            unreachable!()
        };

        let cursor = if state.cursor_visible { "_" } else { "" };

        let text = Line::from(vec![
            Span::from(" ❯ ").fg(PLASTIC_SECONDARY_COLOR),
            Span::from(self.text.clone()).fg(PLASTIC_EMPHASIS_COLOR),
            Span::from(cursor).fg(PLASTIC_SECONDARY_COLOR),
        ]);

        let timer_text = Line::from(format!(" {:.1}s ", self.timer.remaining_secs()));

        Block::default()
            .bg(MAC_PURPLE_MUTED_COLOR)
            .render(remaining_area, buf);
        Block::default()
            .bg(PLASTIC_MEDIUM_BACKGROUND_COLOR)
            .render(elapsed_area, buf);

        text.render(text_area, buf);
        timer_text.render(timer_area, buf);
        block.render(area, buf);
    }
}

fn blink_prompt_system(mut prompt_state: ResMut<PromptState>) {
    prompt_state.cursor_visible = !prompt_state.cursor_visible;
}

fn tick_timer_system(time: Res<Time>, mut prompt: ResMut<Prompt>) {
    prompt.timer.tick(time.delta());
}

fn timer_setup_system(mut prompt: ResMut<Prompt>) {
    prompt.timer = Timer::from_seconds(60.0, TimerMode::Once);
}
