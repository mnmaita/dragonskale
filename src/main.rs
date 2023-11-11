use audio::AudioPlugin;
use bevy::prelude::*;
use textures::TexturesPlugin;

mod audio;
mod textures;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        // FIXME: Remove setting the backend explicitly to avoid noisy warnings
        // when https://github.com/gfx-rs/wgpu/issues/3959 gets fixed.
        DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::DX12),
                ..default()
            }),
        }),
        AudioPlugin,
        TexturesPlugin,
    ));


    app.run();
}
