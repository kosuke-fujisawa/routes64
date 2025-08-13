use crate::app::boot::*;
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
        (setup_camera, start_resource_loading, setup_save_manager).chain(),
    );

    app.add_systems(
        Update,
        check_resources_loaded.run_if(in_state(AppState::Boot)),
    );

    app.add_systems(
        OnEnter(AppState::Title),
        (
            cleanup_ui::<PlayingUI>,
            cleanup_ui::<EndingUI>,
            cleanup_ui::<BackgroundSprite>,
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
            handle_begin_or_continue.run_if(resource_exists::<Current>),
            handle_make_choice.run_if(resource_exists::<Current>),
            handle_restart,
            auto_save_system.run_if(in_state(AppState::Playing)),
            button_interaction_system,
            update_background.run_if(resource_exists::<ScenarioData>),
        ),
    );

    app
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_save_manager(mut commands: Commands) {
    let save_manager = SaveManager::new().expect("Failed to initialize save manager");
    commands.insert_resource(save_manager);
}

type BeginButtonQuery<'w, 's> = Query<'w, 's, &'static Interaction, (Changed<Interaction>, With<BeginNewButton>)>;
type ContinueButtonQuery<'w, 's> = Query<'w, 's, &'static Interaction, (Changed<Interaction>, With<ContinueButton>, Without<crate::ui::components::Disabled>)>;

fn title_button_system(
    mut begin_new_events: EventWriter<BeginNewGame>,
    mut continue_events: EventWriter<ContinueGame>,
    begin_button_query: BeginButtonQuery,
    continue_button_query: ContinueButtonQuery,
    save_manager: Res<SaveManager>,
) {
    // Begin New ボタンの判定
    for interaction in begin_button_query.iter() {
        if *interaction == Interaction::Pressed {
            begin_new_events.send(BeginNewGame);
            return;
        }
    }

    // Continue ボタンの判定（セーブがある場合のみ）
    for interaction in continue_button_query.iter() {
        if *interaction == Interaction::Pressed && save_manager.has_save() {
            continue_events.send(ContinueGame);
            return;
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

fn handle_begin_or_continue(
    mut begin_new_events: EventReader<BeginNewGame>,
    mut continue_events: EventReader<ContinueGame>,
    mut current: ResMut<Current>,
    mut next_state: ResMut<NextState<AppState>>,
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
}

fn handle_make_choice(
    mut choice_events: EventReader<MakeChoice>,
    mut current: ResMut<Current>,
    mut next_state: ResMut<NextState<AppState>>,
    scenario_data: Res<ScenarioData>,
) {
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
}

fn handle_restart(
    mut restart_events: EventReader<RestartGame>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _event in restart_events.read() {
        next_state.set(AppState::Title);
        info!("Restarting game");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_systems_registration() {
        // システムが登録されていることのみをテスト（実行はしない）
        let app = create_app();

        // リソース登録の確認
        assert!(app.world().get_resource::<NextState<AppState>>().is_some());

        // イベント登録の確認（EVENTSタイプを直接確認するのは困難なので、動作による確認）
    }
}
