use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::log::tracing_subscriber::layer::SubscriberExt;
use bevy::log::tracing_subscriber::util::SubscriberInitExt;
use bevy::log::{LogPlugin, tracing_subscriber};
use bevy::prelude::*;
use bevy::winit::WinitPlugin;

pub(super) fn plugin(app: &mut App) {
    tracing_subscriber::registry()
        .with(tui_logger::TuiTracingSubscriberLayer)
        .init();
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();

    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .disable::<WinitPlugin>()
            .disable::<LogPlugin>(),
        ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(1. / 90.)),
    ));
}
