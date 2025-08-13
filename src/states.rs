use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Boot,
    Title,
    Playing,
    Ending,
}

#[derive(Component)]
pub struct TitleUI;

#[derive(Component)]
pub struct PlayingUI;

#[derive(Component)]
pub struct EndingUI;

#[derive(Component)]
pub struct BeginNewButton;

#[derive(Component)]
pub struct ContinueButton;

#[derive(Component)]
pub struct ChoiceButton {
    pub choice_index: usize,
}

#[derive(Component)]
pub struct RestartButton;

#[derive(Event)]
pub struct BeginNewGame;

#[derive(Event)]
pub struct ContinueGame;

#[derive(Event)]
pub struct MakeChoice {
    pub choice_index: usize,
}

#[derive(Event)]
pub struct RestartGame;

pub fn cleanup_ui<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        assert_eq!(AppState::default(), AppState::Boot);
    }

    #[test]
    fn test_app_state_transitions() {
        let states = [AppState::Boot, AppState::Title, AppState::Playing, AppState::Ending];
        for state in states {
            match state {
                AppState::Boot => assert_ne!(state, AppState::Title),
                AppState::Title => assert_ne!(state, AppState::Playing),
                AppState::Playing => assert_ne!(state, AppState::Ending),
                AppState::Ending => assert_ne!(state, AppState::Boot),
            }
        }
    }
}
