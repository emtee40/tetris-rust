use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{GameState, GameStats};

pub struct AudioPlugin;

#[derive(Deref, DerefMut)]
struct MusicInstanceHandle(Handle<AudioInstance>);

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kira_audio::AudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(play_music))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(stop_music))
            .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(pause_music))
            .add_system_set(SystemSet::on_exit(GameState::Paused).with_system(unpause_music))
            .add_system_set(
                SystemSet::on_update(GameState::InGame).with_system(update_playback_speed),
            );
    }
}

fn play_music(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let handle = audio
        .play(asset_server.load("sounds/Crowander - Gypsy.mp3"))
        .with_playback_rate(0.9)
        .looped()
        .handle();
    commands.insert_resource(MusicInstanceHandle(handle));
}

fn stop_music(
    handle: Res<MusicInstanceHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&handle) {
        instance.stop(AudioTween::default());
    }
}

fn pause_music(
    handle: Res<MusicInstanceHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&handle) {
        instance.pause(AudioTween::default());
    }
}

fn unpause_music(
    handle: Res<MusicInstanceHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&handle) {
        instance.resume(AudioTween::default());
    }
}

fn update_playback_speed(
    handle: Res<MusicInstanceHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    stats: Res<GameStats>,
) {
    if stats.is_changed() {
        let speed = 0.9 + stats.shapes_spawned as f64 * 0.005;

        if let Some(instance) = audio_instances.get_mut(&handle) {
            instance.set_playback_rate(speed, AudioTween::default());
        }
    }
}
