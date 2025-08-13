use bevy::audio::Volume;
use bevy::prelude::*;

#[derive(Resource)]
pub struct RainAudioHandle(pub Handle<AudioSource>);

pub fn start_rain_loop(
    mut commands: Commands,
    rain_handle: Option<Res<RainAudioHandle>>,
    audio_query: Query<Entity, With<RainLoop>>,
) {
    let Some(rain_handle) = rain_handle else { return; };
    
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

#[derive(Component)]
pub struct RainLoop;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rain_audio_handle_creation() {
        let rain_audio: Handle<AudioSource> = Handle::weak_from_u128(12345);
        let rain_handle = RainAudioHandle(rain_audio);

        assert!(rain_handle.0.is_weak());
    }
}
