use crate::audio::*;
use crate::save::*;
use crate::scenario::*;
use crate::states::*;
use crate::ui::*;
use bevy::prelude::*;

pub fn create_app() -> App {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "routes64".to_string(),
            resolution: (1280.0, 720.0).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }));

    app.init_state::<AppState>();

    app.add_event::<BeginNewGame>();
    app.add_event::<ContinueGame>();
    app.add_event::<MakeChoice>();
    app.add_event::<RestartGame>();

    app.add_systems(
        Startup,
        (setup_camera, preload_resources, setup_save_manager, transition_to_title).chain(),
    );

    app.add_systems(
        OnEnter(AppState::Title),
        (
            cleanup_ui::<PlayingUI>,
            cleanup_ui::<EndingUI>,
            setup_background,
            start_rain_loop,
            setup_title_ui,
        )
            .chain(),
    );

    app.add_systems(
        OnEnter(AppState::Playing),
        (
            cleanup_ui::<TitleUI>,
            cleanup_ui::<EndingUI>,
            setup_playing_ui,
        )
            .chain(),
    );

    app.add_systems(
        OnEnter(AppState::Ending),
        (cleanup_ui::<PlayingUI>, setup_ending_ui).chain(),
    );

    app.add_systems(
        Update,
        (
            title_button_system.run_if(in_state(AppState::Title)),
            playing_button_system.run_if(in_state(AppState::Playing)),
            ending_button_system.run_if(in_state(AppState::Ending)),
            game_event_system,
            auto_save_system.run_if(in_state(AppState::Playing)),
            button_interaction_system,
            update_background,
        ),
    );

    app
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn preload_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scenario_json =
        std::fs::read_to_string("assets/scenario.json").expect("Failed to read scenario.json");

    let scenario_data =
        ScenarioData::load_from_json(&scenario_json).expect("Failed to load scenario data");

    commands.insert_resource(scenario_data);
    commands.insert_resource(Current::default());

    let font_handle = asset_server.load("fonts/NotoSansJP-Regular.ttf");
    commands.insert_resource(GameFont(font_handle));

    // Skip rain audio for now to avoid format issues
    // let rain_handle = asset_server.load("audio/rain.ogg");
    // commands.insert_resource(RainAudioHandle(rain_handle));

    info!("Resources loaded successfully");
}

fn transition_to_title(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Title);
}

fn setup_save_manager(mut commands: Commands) {
    let save_manager = SaveManager::new().expect("Failed to initialize save manager");
    commands.insert_resource(save_manager);
}

fn title_button_system(
    mut begin_new_events: EventWriter<BeginNewGame>,
    mut continue_events: EventWriter<ContinueGame>,
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    begin_button_query: Query<&BeginNewButton>,
    continue_button_query: Query<&ContinueButton>,
    save_manager: Res<SaveManager>,
) {
    for interaction in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if !begin_button_query.is_empty() {
                begin_new_events.send(BeginNewGame);
                return;
            }

            if !continue_button_query.is_empty() && save_manager.has_save() {
                continue_events.send(ContinueGame);
                return;
            }
        }
    }
}

type ChoiceButtonQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static ChoiceButton),
    (Changed<Interaction>, With<Button>),
>;

fn playing_button_system(
    mut choice_events: EventWriter<MakeChoice>,
    mut button_query: ChoiceButtonQuery,
) {
    for (interaction, choice_button) in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            choice_events.send(MakeChoice {
                choice_index: choice_button.choice_index,
            });
        }
    }
}

fn ending_button_system(
    mut restart_events: EventWriter<RestartGame>,
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
) {
    for interaction in button_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            restart_events.send(RestartGame);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn game_event_system(
    mut begin_new_events: EventReader<BeginNewGame>,
    mut continue_events: EventReader<ContinueGame>,
    mut choice_events: EventReader<MakeChoice>,
    mut restart_events: EventReader<RestartGame>,
    mut current: ResMut<Current>,
    mut next_state: ResMut<NextState<AppState>>,
    scenario_data: Res<ScenarioData>,
    save_manager: Res<SaveManager>,
) {
    for _event in begin_new_events.read() {
        *current = Current::default();
        next_state.set(AppState::Playing);
        info!("Starting new game");
    }

    for _event in continue_events.read() {
        if let Ok(Some(saved_current)) = save_manager.load() {
            *current = saved_current;
            next_state.set(AppState::Playing);
            info!("Continuing from save");
        }
    }

    for choice_event in choice_events.read() {
        match scenario_data.transition(&current, choice_event.choice_index) {
            Ok(new_current) => {
                *current = new_current;

                if scenario_data.is_ending(&current) {
                    next_state.set(AppState::Ending);
                    info!("Reached ending: {}", current.id);
                } else {
                    info!("Transitioned to: {}", current.id);
                }
            }
            Err(e) => {
                error!("Failed to make choice: {}", e);
            }
        }
    }

    for _event in restart_events.read() {
        next_state.set(AppState::Title);
        info!("Restarting game");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_app() {
        let app = create_app();
        assert!(app.world().get_resource::<State<AppState>>().is_some());
    }

    #[test]
    fn test_app_has_required_systems() {
        let mut app = create_app();

        app.update();
    }
}
