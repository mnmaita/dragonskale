use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{combat::CombatPlugin, enemy::EnemyPlugin, level::LevelPlugin, player::PlayerPlugin};

pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CombatPlugin)
            .add(EnemyPlugin)
            .add(LevelPlugin)
            .add(PlayerPlugin)
    }
}
