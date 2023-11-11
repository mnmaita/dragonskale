use audio::AudioPlugin;
use bevy::prelude::*;

mod audio;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, AudioPlugin));

    app.run();
}
