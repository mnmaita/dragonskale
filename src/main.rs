use audio::AudioPlugin;
use bevy::prelude::*;
use textures::TexturesPlugin;

mod audio;
mod textures;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, AudioPlugin, TexturesPlugin));

    app.run();
}
