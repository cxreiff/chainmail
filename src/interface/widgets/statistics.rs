use bevy::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Widget},
};

use crate::{
    constants::{MAC_GREEN_MUTED_COLOR, PLASTIC_PRIMARY_COLOR, PLASTIC_SECONDARY_COLOR},
    score::Statistics,
};

#[derive(Deref, DerefMut, Debug)]
pub struct StatisticsWidget<'a>(pub &'a Statistics);

impl Widget for StatisticsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block = Block::default()
            .bg(MAC_GREEN_MUTED_COLOR)
            .fg(PLASTIC_PRIMARY_COLOR);

        let [left_area, center_area, right_area] =
            *Layout::horizontal(Constraint::from_fills([1, 1, 1]))
                .spacing(1)
                .split(area)
        else {
            unreachable!()
        };

        let score_label = if area.width > 40 { "score: " } else { "s: " };
        let score_line = Line::from(vec![
            Span::from(score_label).fg(PLASTIC_SECONDARY_COLOR),
            Span::from(self.score.to_string()),
        ])
        .centered();

        let money_label = if area.width > 40 { "money: " } else { "m: " };
        let money_line = Line::from(vec![
            Span::from(money_label).fg(PLASTIC_SECONDARY_COLOR),
            Span::from(self.money.to_string()),
        ])
        .centered();

        let income_label = if area.width > 40 { "income: " } else { "i: " };
        let income_line = Line::from(vec![
            Span::from(income_label).fg(PLASTIC_SECONDARY_COLOR),
            Span::from(self.income.to_string()),
        ])
        .centered();

        let inner_left_area = block.inner(left_area);
        let inner_center_area = block.inner(center_area);
        let inner_right_area = block.inner(right_area);

        block.clone().render(left_area, buf);
        block.clone().render(center_area, buf);
        block.render(right_area, buf);
        score_line.render(inner_left_area, buf);
        money_line.render(inner_center_area, buf);
        income_line.render(inner_right_area, buf);
    }
}
