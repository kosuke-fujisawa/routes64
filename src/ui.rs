use crate::save::SaveManager;
use crate::scenario::{Current, ScenarioData};
use crate::states::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario_data: Option<Res<ScenarioData>>,
) {
    let Some(scenario_data) = scenario_data else { return; };
    
    let background_handle: Handle<Image> =
        asset_server.load(&scenario_data.scenario.meta.default_background);

    commands.spawn((
        SpriteBundle {
            texture: background_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        BackgroundSprite,
    ));
}

#[derive(Component)]
pub struct BackgroundSprite;

pub fn setup_title_ui(
    mut commands: Commands,
    font: Res<GameFont>,
    save_manager: Res<SaveManager>,
    scenario_data: Res<ScenarioData>,
) {
    let has_save = save_manager.has_save();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            TitleUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &scenario_data.scenario.meta.title,
                TextStyle {
                    font: font.0.clone(),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    },
                    BeginNewButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "はじめから",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(200.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: if has_save {
                            Color::srgb(0.3, 0.3, 0.3)
                        } else {
                            Color::srgb(0.1, 0.1, 0.1)
                        }
                        .into(),
                        ..default()
                    },
                    ContinueButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "前回の続きから",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 24.0,
                            color: if has_save {
                                Color::WHITE
                            } else {
                                Color::srgb(0.5, 0.5, 0.5)
                            },
                        },
                    ));
                });
        });
}

pub fn setup_playing_ui(
    mut commands: Commands,
    font: Res<GameFont>,
    scenario_data: Res<ScenarioData>,
    current: Res<Current>,
) {
    let node = scenario_data.get_node(&current.id).unwrap();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(30.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            PlayingUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        max_width: Val::Px(600.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.2, 0.8).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            &node.text,
                            TextStyle {
                                font: font.0.clone(),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        ),
                        GameText,
                    ));
                });

            if !node.choices.is_empty() {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        for (i, choice) in node.choices.iter().enumerate() {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(60.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            padding: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.4, 0.4, 0.6).into(),
                                        ..default()
                                    },
                                    ChoiceButton { choice_index: i },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        &choice.label,
                                        TextStyle {
                                            font: font.0.clone(),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                        }
                    });
            }
        });
}

pub fn setup_ending_ui(
    mut commands: Commands,
    font: Res<GameFont>,
    scenario_data: Res<ScenarioData>,
    current: Res<Current>,
) {
    let node = scenario_data.get_node(&current.id).unwrap();
    let ending = node.ending.as_ref().unwrap();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(30.0),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            EndingUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        max_width: Val::Px(600.0),
                        padding: UiRect::all(Val::Px(30.0)),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    background_color: Color::srgba(0.2, 0.2, 0.2, 0.9).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        &node.text,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        format!("ルートID: {}", current.id),
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 18.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        &ending.tag,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 32.0,
                            color: Color::srgb(1.0, 0.8, 0.0),
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.6, 0.4, 0.4).into(),
                        ..default()
                    },
                    RestartButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "もう一度",
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn update_background(
    mut background_query: Query<&mut Handle<Image>, With<BackgroundSprite>>,
    asset_server: Res<AssetServer>,
    scenario_data: Res<ScenarioData>,
    current: Res<Current>,
) {
    if current.is_changed() {
        if let Ok(mut background_handle) = background_query.get_single_mut() {
            let new_bg = if let Some(node) = scenario_data.get_node(&current.id) {
                node.bg
                    .as_ref()
                    .unwrap_or(&scenario_data.scenario.meta.default_background)
            } else {
                &scenario_data.scenario.meta.default_background
            };

            *background_handle = asset_server.load(new_bg);
        }
    }
}

type ButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<Button>),
>;

pub fn button_interaction_system(mut interaction_query: ButtonInteractionQuery) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::srgb(0.6, 0.6, 0.6).into();
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.5, 0.5, 0.5).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.3, 0.3, 0.3).into();
            }
        }
    }
}

#[derive(Component)]
pub struct GameText;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_font_resource() {
        let font_handle: Handle<Font> = Handle::weak_from_u128(12345);
        let game_font = GameFont(font_handle);

        assert!(game_font.0.is_weak());
    }
}
