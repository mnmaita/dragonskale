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
        app.init_resource::<MusicFolderHandle>();
        app.init_resource::<SoundEffectFolderHandle>();
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

#[derive(Resource, PartialEq)]
struct AudioLoadStates {
    sound_effects_load_state: RecursiveDependencyLoadState,
    music_load_state: RecursiveDependencyLoadState,
}

impl Default for AudioLoadStates {
    fn default() -> Self {
        Self {
            sound_effects_load_state: RecursiveDependencyLoadState::NotLoaded,
            music_load_state: RecursiveDependencyLoadState::NotLoaded,
        }
    }
}

impl AudioLoadStates {
    pub const LOADED: Self = Self {
        music_load_state: RecursiveDependencyLoadState::Loaded,
        sound_effects_load_state: RecursiveDependencyLoadState::Loaded,
    };
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MusicFolderHandle(Handle<LoadedFolder>);

#[derive(Resource, Default, Deref, DerefMut)]
struct SoundEffectFolderHandle(Handle<LoadedFolder>);

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
        let path = format!("{ASSET_FOLDER_MUSIC}/{file_name}");
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
        let path = format!("{ASSET_FOLDER_SFX}/{file_name}");
        let source = asset_server.get_handle(path).unwrap_or_default();
        let mut entity = commands.spawn(AudioBundle { source, settings });

        if let Some(transform) = spatial_transform {
            entity.insert(SpatialBundle::from_transform(*transform));
        }
    }
}

fn load_music_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music_folder_handle = asset_server.load_folder(ASSET_FOLDER_MUSIC);
    commands.insert_resource(MusicFolderHandle(music_folder_handle));
}

fn load_sound_effect_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sound_effects_folder_handle = asset_server.load_folder(ASSET_FOLDER_SFX);
    commands.insert_resource(SoundEffectFolderHandle(sound_effects_folder_handle));
}

fn update_music_assets_load_state(
    mut audio_load_states: ResMut<AudioLoadStates>,
    music_folder_handle: Res<MusicFolderHandle>,
    asset_server: Res<AssetServer>,
) {
    audio_load_states.music_load_state =
        asset_server.recursive_dependency_load_state(music_folder_handle.id());
}

fn update_sound_effect_assets_load_state(
    mut audio_load_states: ResMut<AudioLoadStates>,
    sound_effect_folder_handle: Res<SoundEffectFolderHandle>,
    asset_server: Res<AssetServer>,
) {
    audio_load_states.sound_effects_load_state =
        asset_server.recursive_dependency_load_state(sound_effect_folder_handle.id());
}

pub fn audio_assets_loaded() -> impl Condition<()> {
    IntoSystem::into_system(resource_equals(AudioLoadStates::LOADED))
}
