use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

pub const ASSET_FOLDER_TEXTURES: &str = "textures";

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TexturesLoadState>();
        app.init_resource::<TextureFolderHandle>();
        app.add_systems(Startup, load_textures);
        app.add_systems(
            Update,
            update_texture_assets_load_state.run_if(resource_equals(TexturesLoadState::default())),
        );
    }
}

#[derive(Resource, PartialEq)]
struct TexturesLoadState(RecursiveDependencyLoadState);

impl Default for TexturesLoadState {
    fn default() -> Self {
        Self(RecursiveDependencyLoadState::NotLoaded)
    }
}

impl TexturesLoadState {
    pub const LOADED: Self = Self(RecursiveDependencyLoadState::Loaded);
}

#[derive(Resource, Default, Deref, DerefMut)]
struct TextureFolderHandle(Handle<LoadedFolder>);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures_folder_handle = asset_server.load_folder(ASSET_FOLDER_TEXTURES);
    commands.insert_resource(TextureFolderHandle(textures_folder_handle));
}

fn update_texture_assets_load_state(
    mut textures_load_state: ResMut<TexturesLoadState>,
    textures_folder_handle: Res<TextureFolderHandle>,
    asset_server: Res<AssetServer>,
) {
    textures_load_state.0 =
        asset_server.recursive_dependency_load_state(textures_folder_handle.id());
}

pub fn texture_assets_loaded() -> impl Condition<()> {
    IntoSystem::into_system(resource_equals(TexturesLoadState::LOADED))
}
