use bevy::prelude::*;

use crate::AppState;

use super::{enemy::EnemyPlugin, player::spawn_player};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnemyPlugin);

        app.add_systems(OnEnter(AppState::InGame), spawn_player);
    }
}
