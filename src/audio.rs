#[cfg(feature = "rain_bgm")]
use bevy::audio::Volume;
use bevy::prelude::*;

#[cfg(feature = "rain_bgm")]
#[derive(Resource)]
pub struct RainAudioHandle(pub Handle<AudioSource>);

#[cfg(feature = "rain_bgm")]
pub fn start_rain_loop(
    mut commands: Commands,
    rain_handle: Res<RainAudioHandle>,
    audio_query: Query<Entity, With<RainLoop>>,
) {
    if audio_query.is_empty() {
        commands.spawn((
            AudioBundle {
                source: rain_handle.0.clone(),
                settings: PlaybackSettings::LOOP.with_volume(Volume::new(0.5)),
            },
            RainLoop,
        ));
        info!("Rain loop started");
    }
}

// rain_bgm feature が無効な場合のダミー実装
#[cfg(not(feature = "rain_bgm"))]
pub fn start_rain_loop() {
    // 何もしない（雨音を再生しない）
}

#[cfg(feature = "rain_bgm")]
#[derive(Component)]
pub struct RainLoop;

#[cfg(all(test, feature = "rain_bgm"))]
mod tests {
    use super::*;

    #[test]
    fn test_rain_audio_handle_creation() {
        let rain_audio: Handle<AudioSource> = Handle::weak_from_u128(12345);
        let rain_handle = RainAudioHandle(rain_audio);

        assert!(rain_handle.0.is_weak());
    }
}
