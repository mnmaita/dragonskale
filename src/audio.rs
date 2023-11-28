use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};

pub const ASSET_FOLDER_MUSIC: &str = "music";
pub const ASSET_FOLDER_SFX: &str = "sfx";

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayMusicEvent>();
        app.add_event::<PlaySoundEffectEvent>();
        app.add_event::<StopMusicEvent>();
        app.init_resource::<AudioLoadStates>();
        app.init_resource::<MusicHandles>();
        app.init_resource::<SoundEffectHandles>();
        app.add_systems(Startup, (load_music_files, load_sound_effect_files));
        app.add_systems(
            Update,
            (
                handle_play_music_events,
                handle_play_sound_effect_events,
                handle_stop_music_events,
            )
                .run_if(resource_equals(AudioLoadStates::LOADED)),
        );
        app.add_systems(
            Update,
            (
                update_music_assets_load_state,
                update_sound_effect_assets_load_state,
            )
                .chain()
                .run_if(resource_equals(AudioLoadStates::default())),
        );
    }
}

#[derive(PartialEq)]
enum AudioLoadState {
    NotLoaded,
    Loading,
    Loaded,
    Failed,
}

impl Default for AudioLoadState {
    fn default() -> Self {
        Self::NotLoaded
    }
}

impl From<RecursiveDependencyLoadState> for AudioLoadState {
    fn from(value: RecursiveDependencyLoadState) -> Self {
        match value {
            RecursiveDependencyLoadState::NotLoaded => Self::NotLoaded,
            RecursiveDependencyLoadState::Loading => Self::Loading,
            RecursiveDependencyLoadState::Loaded => Self::Loaded,
            RecursiveDependencyLoadState::Failed => Self::Failed,
        }
    }
}

#[derive(Resource, PartialEq)]
struct AudioLoadStates {
    sound_effects_load_state: AudioLoadState,
    music_load_state: AudioLoadState,
}

impl Default for AudioLoadStates {
    fn default() -> Self {
        Self {
            sound_effects_load_state: AudioLoadState::NotLoaded,
            music_load_state: AudioLoadState::NotLoaded,
        }
    }
}

impl AudioLoadStates {
    pub const LOADED: Self = Self {
        music_load_state: AudioLoadState::Loaded,
        sound_effects_load_state: AudioLoadState::Loaded,
    };
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MusicHandles(
    #[cfg(not(target_family = "wasm"))] Handle<LoadedFolder>,
    #[cfg(target_family = "wasm")] Vec<Handle<AudioSource>>,
);

#[derive(Resource, Default, Deref, DerefMut)]
struct SoundEffectHandles(
    #[cfg(not(target_family = "wasm"))] Handle<LoadedFolder>,
    #[cfg(target_family = "wasm")] Vec<Handle<AudioSource>>,
);

#[derive(Event)]
pub struct PlayMusicEvent {
    file_name: String,
    settings: Option<PlaybackSettings>,
    spatial_transform: Option<Transform>,
}

impl PlayMusicEvent {
    pub fn new(
        file_name: impl Into<String>,
        settings: Option<PlaybackSettings>,
        spatial_transform: Option<Transform>,
    ) -> Self {
        let file_name = file_name.into();
        Self {
            file_name,
            settings,
            spatial_transform,
        }
    }

    pub fn new_with_defaults(file_name: impl Into<String>) -> Self {
        let file_name = file_name.into();
        Self {
            file_name,
            settings: None,
            spatial_transform: None,
        }
    }
}

#[derive(Event)]
pub struct StopMusicEvent;

#[derive(Event)]
pub struct PlaySoundEffectEvent {
    pub file_name: String,
    pub settings: Option<PlaybackSettings>,
    pub spatial_transform: Option<Transform>,
}

impl PlaySoundEffectEvent {
    pub fn new(
        file_name: impl Into<String>,
        settings: Option<PlaybackSettings>,
        spatial_transform: Option<Transform>,
    ) -> Self {
        let file_name = file_name.into();
        Self {
            file_name,
            settings,
            spatial_transform,
        }
    }

    pub fn new_with_defaults(file_name: impl Into<String>) -> Self {
        let file_name = file_name.into();
        Self {
            file_name,
            settings: None,
            spatial_transform: None,
        }
    }
}

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Component)]
pub struct SoundEffect;

fn handle_play_music_events(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut event_reader: EventReader<PlayMusicEvent>,
) {
    for event in event_reader.read() {
        let PlayMusicEvent {
            file_name,
            settings,
            spatial_transform,
        } = event;
        let settings = settings.unwrap_or(PlaybackSettings::DESPAWN);
        let path = format_music_file_name(file_name);
        let source = asset_server.get_handle(path).unwrap_or_default();
        let mut entity = commands.spawn((BackgroundMusic, AudioBundle { source, settings }));

        if let Some(transform) = spatial_transform {
            entity.insert(SpatialBundle::from_transform(*transform));
        }
    }
}

fn handle_stop_music_events(
    mut commands: Commands,
    mut event_reader: EventReader<StopMusicEvent>,
    query: Query<Entity, With<BackgroundMusic>>,
) {
    if !event_reader.is_empty() {
        let entity = query.single();
        commands.entity(entity).despawn_recursive();
    }
    event_reader.clear();
}

fn handle_play_sound_effect_events(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut event_reader: EventReader<PlaySoundEffectEvent>,
) {
    for event in event_reader.read() {
        let PlaySoundEffectEvent {
            file_name,
            settings,
            spatial_transform,
        } = event;
        let settings = settings.unwrap_or(PlaybackSettings::DESPAWN);
        let path = format_sfx_file_name(file_name);
        let source = asset_server.get_handle(path).unwrap_or_default();
        let mut entity = commands.spawn((AudioBundle { source, settings }, SoundEffect));

        if let Some(transform) = spatial_transform {
            entity.insert(SpatialBundle::from_transform(*transform));
        }
    }
}

fn load_music_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music_handles = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.load_folder(ASSET_FOLDER_MUSIC)
        }

        #[cfg(target_family = "wasm")]
        {
            let asset_music_list = [
                format_music_file_name("theme1.ogg"),
                format_music_file_name("theme2.ogg"),
                format_music_file_name("theme3.ogg"),
            ];
            asset_music_list
                .iter()
                .map(|path| asset_server.load::<AudioSource>(path))
                .collect::<Vec<Handle<AudioSource>>>()
        }
    };

    commands.insert_resource(MusicHandles(music_handles));
}

fn load_sound_effect_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sound_effect_handles = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.load_folder(ASSET_FOLDER_SFX)
        }

        #[cfg(target_family = "wasm")]
        {
            let asset_sfx_list = [
                format_sfx_file_name("breathend.ogg"),
                format_sfx_file_name("breathloop.ogg"),
                format_sfx_file_name("breathstart.ogg"),
            ];
            asset_sfx_list
                .iter()
                .map(|path| asset_server.load::<AudioSource>(path))
                .collect::<Vec<Handle<AudioSource>>>()
        }
    };

    commands.insert_resource(SoundEffectHandles(sound_effect_handles));
}

fn update_music_assets_load_state(
    mut audio_load_states: ResMut<AudioLoadStates>,
    music_handles: Res<MusicHandles>,
    asset_server: Res<AssetServer>,
) {
    audio_load_states.music_load_state = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server
                .recursive_dependency_load_state(music_handles.id())
                .into()
        }
        #[cfg(target_family = "wasm")]
        {
            let all_loaded = music_handles.iter().all(|handle| {
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

fn update_sound_effect_assets_load_state(
    mut audio_load_states: ResMut<AudioLoadStates>,
    sound_effect_handles: Res<SoundEffectHandles>,
    asset_server: Res<AssetServer>,
) {
    audio_load_states.sound_effects_load_state = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server
                .recursive_dependency_load_state(sound_effect_handles.id())
                .into()
        }
        #[cfg(target_family = "wasm")]
        {
            let all_loaded = sound_effect_handles.iter().all(|handle| {
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

pub fn audio_assets_loaded() -> impl Condition<()> {
    IntoSystem::into_system(resource_equals(AudioLoadStates::LOADED))
}

fn format_music_file_name(file_name: &str) -> String {
    format!("{ASSET_FOLDER_MUSIC}/{file_name}")
}

fn format_sfx_file_name(file_name: &str) -> String {
    format!("{ASSET_FOLDER_SFX}/{file_name}")
}
