use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    combat::CombatPlugin, enemy::EnemyPlugin, hud::HudPlugin, level::LevelPlugin,
    player::PlayerPlugin,
};

pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CombatPlugin)
            .add(EnemyPlugin)
            .add(HudPlugin)
            .add(LevelPlugin)
            .add(PlayerPlugin)
    }
}
