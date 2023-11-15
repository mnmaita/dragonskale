use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{enemy::EnemyPlugin, level::LevelPlugin, player::PlayerPlugin};

pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EnemyPlugin)
            .add(LevelPlugin)
            .add(PlayerPlugin)
    }
}
