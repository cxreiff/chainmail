use bevy::{
    diagnostic::{DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use ratatui::widgets::{Borders, Padding, Widget};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType},
};
use tui_logger::TuiLoggerWidget;

use crate::{
    constants::{
        MAC_GREEN_MUTED_COLOR, MAC_PURPLE_MUTED_COLOR, MAC_RED_MUTED_COLOR,
        PLASTIC_DARK_BACKGROUND_COLOR, PLASTIC_LIGHT_BACKGROUND_COLOR, PLASTIC_PRIMARY_COLOR,
    },
    states::Statistics,
};

use super::{draw::Flags, widgets::statistics::StatisticsWidget};

pub fn layout_frame(
    frame: &mut Frame,
    flags: &Flags,
    diagnostics: &DiagnosticsStore,
    stats: &Statistics,
    show_log_panel: bool,
) -> ratatui::layout::Rect {
    Block::default()
        .bg(PLASTIC_DARK_BACKGROUND_COLOR)
        .render(frame.area(), frame.buffer_mut());

    let main_block = Block::bordered()
        .border_type(BorderType::QuadrantInside)
        .border_style(Style::default().bg(PLASTIC_DARK_BACKGROUND_COLOR))
        .bg(PLASTIC_LIGHT_BACKGROUND_COLOR)
        .fg(PLASTIC_PRIMARY_COLOR);
    let undertab_block = Block::bordered()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_type(BorderType::QuadrantInside)
        .padding(Padding::horizontal(2))
        .bg(PLASTIC_DARK_BACKGROUND_COLOR)
        .fg(PLASTIC_PRIMARY_COLOR);

    let [main_area, bottom_area] = *Layout::new(
        Direction::Vertical,
        [Constraint::Fill(1), Constraint::Length(2)],
    )
    .split(frame.area()) else {
        unreachable!()
    };

    let name_string = "CHAINMAIL";
    let name_line = Line::from(name_string).centered();

    let mut status_strings = vec![];
    status_strings.push(format!("sound: {}", if flags.sound { "ON" } else { "OFF" }));
    if flags.debug {
        if let Some(value) = diagnostics
            .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
            .and_then(|count| count.value())
        {
            status_strings.push(format!("entities: {value}"));
        }

        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            status_strings.push(format!("fps: {value:3.0}"));
        }
    }
    let status_string = status_strings.join("  |  ");
    let status_line = Line::from(status_string).centered();

    let stats_widget = StatisticsWidget(stats);

    let controls_string = ["1 to debug", "2 to toggle sound"].join("  |  ");
    let controls_line = Line::from(controls_string.clone()).centered();

    let bottom_area = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Length(name_string.len() as u16 + 8),
            Constraint::Fill(1),
            Constraint::Length(controls_string.len() as u16 + 8),
        ],
    )
    .spacing(1)
    .split(bottom_area);
    frame.render_widget(
        undertab_block
            .clone()
            .border_style(Style::default().bg(PLASTIC_DARK_BACKGROUND_COLOR))
            .bg(MAC_RED_MUTED_COLOR),
        bottom_area[0],
    );
    frame.render_widget(name_line, undertab_block.inner(bottom_area[0]));
    frame.render_widget(
        undertab_block
            .clone()
            .border_style(Style::default().bg(PLASTIC_DARK_BACKGROUND_COLOR))
            .bg(MAC_GREEN_MUTED_COLOR),
        bottom_area[1],
    );

    if flags.debug {
        frame.render_widget(status_line, undertab_block.inner(bottom_area[1]));
    } else {
        frame.render_widget(stats_widget, undertab_block.inner(bottom_area[1]));
    }

    frame.render_widget(
        undertab_block
            .clone()
            .border_style(Style::default().bg(PLASTIC_DARK_BACKGROUND_COLOR))
            .bg(MAC_PURPLE_MUTED_COLOR),
        bottom_area[2],
    );
    frame.render_widget(controls_line, undertab_block.inner(bottom_area[2]));

    if flags.debug {
        let debug_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Fill(2),
                Constraint::Fill(if show_log_panel { 1 } else { 0 }),
            ],
        )
        .split(main_area);

        let inner = main_block.inner(debug_layout[0]);
        frame.render_widget(main_block, debug_layout[0]);

        if show_log_panel {
            frame.render_widget(
                TuiLoggerWidget::default()
                    .block(undertab_block.clone().padding(Padding::uniform(1)))
                    .style(Style::default().bg(ratatui::style::Color::Reset)),
                debug_layout[1],
            );
        }

        inner
    } else {
        let inner = main_block.inner(main_area);
        frame.render_widget(main_block, main_area);

        inner
    }
}
