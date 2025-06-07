use bevy::prelude::*;

use crate::states::GameStates;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Statistics>()
        .add_systems(OnEnter(GameStates::Resetting), payroll_system);
}

#[derive(Resource, Default, Debug)]
pub struct Statistics {
    pub score: i32,
    pub money: i32,
    pub income: i32,
}

fn payroll_system(mut stats: ResMut<Statistics>) {
    stats.money += stats.income;
}
