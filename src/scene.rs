use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_ratatui::RatatuiContext;
use rand::seq::SliceRandom;
use rand_chacha::{
    ChaCha8Rng,
    rand_core::{RngCore, SeedableRng},
};
use ratatui::layout::Size;

static CHARACTERS: &[char] = &['@', '#', '%', '&', '*', '=', '$'];

pub fn plugin(app: &mut App) {
    app.init_resource::<StarRng>().add_systems(
        Update,
        (
            star_spawn_system.run_if(on_timer(Duration::from_millis(100))),
            star_move_system.run_if(on_timer(Duration::from_millis(50))),
        ),
    );
}

#[derive(Component, Debug, Default)]
pub struct Star {
    pub row: u16,
    pub col: u16,
    pub color: ratatui::style::Color,
    pub character: char,
}

#[derive(Resource, Deref, DerefMut)]
pub struct StarRng(ChaCha8Rng);

impl Default for StarRng {
    fn default() -> Self {
        Self(ChaCha8Rng::seed_from_u64(19878367467712))
    }
}

fn star_spawn_system(
    mut commands: Commands,
    ratatui: Res<RatatuiContext>,
    mut rng: ResMut<StarRng>,
) -> Result {
    let Size { width, height } = ratatui.size()?;
    commands.spawn(Star {
        row: (rng.next_u32() % height as u32) as u16,
        col: (rng.next_u32() % width as u32) as u16,
        color: ratatui::style::Color::Rgb(
            (rng.next_u32() % 256) as u8,
            (rng.next_u32() % 256) as u8,
            (rng.next_u32() % 256) as u8,
        ),
        character: *CHARACTERS.choose(&mut rng.0).unwrap(),
    });

    Ok(())
}

fn star_move_system(mut stars: Query<&mut Star>, mut rng: ResMut<StarRng>) {
    for mut star in &mut stars {
        let x_delta = (rng.next_u32() % 3) as i32 - 1;
        let y_delta = (rng.next_u32() % 3) as i32 - 1;
        star.row = (star.row as i32 + y_delta).max(0) as u16;
        star.col = (star.col as i32 + x_delta).max(0) as u16;
    }
}
