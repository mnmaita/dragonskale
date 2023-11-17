use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

pub const ASSET_FOLDER_FONTS: &str = "fonts";

pub struct FontsPlugin;

impl Plugin for FontsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FontsLoadState>();
        app.init_resource::<FontsFolderHandle>();
        app.add_systems(Startup, load_fonts);
        app.add_systems(
            Update,
            update_font_assets_load_state.run_if(resource_equals(FontsLoadState::default())),
        );
    }
}

#[derive(Resource, PartialEq)]
struct FontsLoadState(RecursiveDependencyLoadState);

impl Default for FontsLoadState {
    fn default() -> Self {
        Self(RecursiveDependencyLoadState::NotLoaded)
    }
}

impl FontsLoadState {
    pub const LOADED: Self = Self(RecursiveDependencyLoadState::Loaded);
}

#[derive(Resource, Default, Deref, DerefMut)]
struct FontsFolderHandle(Handle<LoadedFolder>);

fn load_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fonts_folder_handle = asset_server.load_folder(ASSET_FOLDER_FONTS);
    commands.insert_resource(FontsFolderHandle(fonts_folder_handle));
}

fn update_font_assets_load_state(
    mut fonts_load_state: ResMut<FontsLoadState>,
    fonts_folder_handle: Res<FontsFolderHandle>,
    asset_server: Res<AssetServer>,
) {
    fonts_load_state.0 = asset_server.recursive_dependency_load_state(fonts_folder_handle.id());
}

pub fn font_assets_loaded() -> impl Condition<()> {
    IntoSystem::into_system(resource_equals(FontsLoadState::LOADED))
}
