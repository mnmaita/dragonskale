use bevy::{app::PluginGroupBuilder, prelude::*};

use super::{
    combat::CombatPlugin, enemy::EnemyPlugin, fire_breath::FireBreathPlugin, hud::HudPlugin,
    level::LevelPlugin, player::PlayerPlugin,
};

pub struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CombatPlugin)
            .add(EnemyPlugin)
            .add(FireBreathPlugin)
            .add(HudPlugin)
            .add(LevelPlugin)
            .add(PlayerPlugin)
    }
}
