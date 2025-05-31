use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_ratatui::RatatuiContext;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    widgets::Block,
};

use crate::scene::Star;

pub fn plugin(app: &mut App) {
    app.init_resource::<Flags>()
        .add_systems(Update, draw_system);
}

#[derive(Resource, Default)]
pub struct Flags {
    pub debug: bool,
}

fn draw_system(
    mut ratatui: ResMut<RatatuiContext>,
    stars: Query<&Star>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
) -> Result {
    ratatui.draw(|frame| {
        let area = debug_frame(frame, &flags, &diagnostics);

        let buffer = frame.buffer_mut();
        for star in &stars {
            if !area.contains((star.col, star.row).into()) {
                continue;
            }

            let Some(cell) = buffer.cell_mut((star.col, star.row)) else {
                continue;
            };

            cell.fg = star.color;
            cell.set_char(star.character);
        }
    })?;

    Ok(())
}

pub fn debug_frame(frame: &mut Frame, flags: &Flags, diagnostics: &DiagnosticsStore) -> Rect {
    let mut block = Block::bordered()
        .bg(ratatui::style::Color::Black)
        .border_style(Style::default().bg(ratatui::style::Color::Black))
        .title_alignment(Alignment::Center);

    if flags.debug {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            block = block.title_top(format!("[fps: {value:.0}]"));
        }
    }

    let inner = block.inner(frame.area());
    frame.render_widget(block, frame.area());

    inner
}
