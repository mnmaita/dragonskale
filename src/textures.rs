use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

pub const ASSET_FOLDER_TEXTURES: &str = "textures";

pub struct TexturesPlugin;

impl Plugin for TexturesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TexturesLoadState>();
        app.init_resource::<TextureHandles>();
        app.add_systems(Startup, load_textures);
        app.add_systems(
            Update,
            update_texture_assets_load_state
                .run_if(not(resource_equals(TexturesLoadState::Loaded))),
        );
    }
}

#[derive(Resource, PartialEq)]
enum TexturesLoadState {
    NotLoaded,
    Loading,
    Loaded,
    Failed,
}

impl Default for TexturesLoadState {
    fn default() -> Self {
        Self::NotLoaded
    }
}

impl From<RecursiveDependencyLoadState> for TexturesLoadState {
    fn from(value: RecursiveDependencyLoadState) -> Self {
        match value {
            RecursiveDependencyLoadState::NotLoaded => Self::NotLoaded,
            RecursiveDependencyLoadState::Loading => Self::Loading,
            RecursiveDependencyLoadState::Loaded => Self::Loaded,
            RecursiveDependencyLoadState::Failed(_) => Self::Failed,
        }
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct TextureHandles(
    #[cfg(not(target_family = "wasm"))] Handle<LoadedFolder>,
    #[cfg(target_family = "wasm")] Vec<Handle<Image>>,
);

fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handles = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.load_folder(ASSET_FOLDER_TEXTURES)
        }

        #[cfg(target_family = "wasm")]
        {
            let asset_textures_list = [
                format!("{ASSET_FOLDER_TEXTURES}/dragon.png"),
                format!("{ASSET_FOLDER_TEXTURES}/enemy_archer.png"),
                format!("{ASSET_FOLDER_TEXTURES}/enemy_axe.png"),
                format!("{ASSET_FOLDER_TEXTURES}/fire_anim_washed.png"),
                format!("{ASSET_FOLDER_TEXTURES}/fire_anim.png"),
                format!("{ASSET_FOLDER_TEXTURES}/fire_stretch.png"),
                format!("{ASSET_FOLDER_TEXTURES}/fire_wide.png"),
                format!("{ASSET_FOLDER_TEXTURES}/menu_background.png"),
                format!("{ASSET_FOLDER_TEXTURES}/scale_anim.png"),
                format!("{ASSET_FOLDER_TEXTURES}/tileset_ground.png"),
                format!("{ASSET_FOLDER_TEXTURES}/tileset_objects.png"),
            ];
            asset_textures_list
                .iter()
                .map(|path| asset_server.load::<Image>(path))
                .collect::<Vec<Handle<Image>>>()
        }
    };

    commands.insert_resource(TextureHandles(texture_handles));
}

fn update_texture_assets_load_state(
    mut textures_load_state: ResMut<TexturesLoadState>,
    texture_handles: Res<TextureHandles>,
    asset_server: Res<AssetServer>,
) {
    *textures_load_state = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server
                .recursive_dependency_load_state(texture_handles.id())
                .into()
        }
        #[cfg(target_family = "wasm")]
        {
            let all_loaded = texture_handles.iter().all(|handle| {
                asset_server.recursive_dependency_load_state(handle.id())
                    == RecursiveDependencyLoadState::Loaded
            });
            if all_loaded {
                RecursiveDependencyLoadState::Loaded.into()
            } else {
                RecursiveDependencyLoadState::NotLoaded.into()
            }
        }
    };
}

pub fn texture_assets_loaded() -> impl Condition<()> {
    IntoSystem::into_system(resource_equals(TexturesLoadState::Loaded))
}
