use crate::app::constants::ui::*;
use crate::save::SaveManager;
use crate::scenario::{Current, ScenarioData};
use crate::states::*;
use crate::ui::components::{
    create_button_text_style, create_game_button, create_game_button_with_color, Disabled,
};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameFont(pub Handle<Font>);

/// 背景スプライトを無条件で作成する（直接呼び出し用）
///
/// 現在は setup_background_if_needed() を使用しているため直接の呼び出しはないが、
/// 将来的に背景の強制再作成が必要になった場合のために保持している。
/// 例: 背景画像の動的切り替え、デバッグ時の背景リセットなど
#[allow(dead_code)]
pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario_data: Res<ScenarioData>,
) {
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

/// 背景スプライトが存在しない場合のみ作成する（一意性保証）
pub fn setup_background_if_needed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario_data: Res<ScenarioData>,
    background_query: Query<Entity, With<BackgroundSprite>>,
) {
    // 既に背景スプライトが存在する場合は何もしない
    if background_query.is_empty() {
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
        info!("Background sprite created");
    } else {
        debug!("Background sprite already exists, skipping creation");
    }
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
                    font_size: TITLE_FONT_SIZE,
                    color: TEXT_NORMAL_COLOR,
                },
            ));

            parent
                .spawn((create_game_button(), BeginNewButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "はじめから",
                        create_button_text_style(font.0.clone(), BUTTON_FONT_SIZE),
                    ));
                });

            let mut continue_entity = parent.spawn((
                create_game_button_with_color(if has_save {
                    BUTTON_NORMAL_COLOR
                } else {
                    BUTTON_DISABLED_COLOR
                }),
                ContinueButton,
            ));

            // セーブがない場合はDisabledコンポーネントを追加
            if !has_save {
                continue_entity.insert(Disabled);
            }

            continue_entity.with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "前回の続きから",
                    TextStyle {
                        font: font.0.clone(),
                        font_size: BUTTON_FONT_SIZE,
                        color: if has_save {
                            TEXT_NORMAL_COLOR
                        } else {
                            TEXT_DISABLED_COLOR
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
    let node = scenario_data.get_node_or_fallback(&current.id);

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
                                font_size: GAME_TEXT_FONT_SIZE,
                                color: TEXT_NORMAL_COLOR,
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
                                            font_size: CHOICE_FONT_SIZE,
                                            color: TEXT_NORMAL_COLOR,
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
    let node = scenario_data.get_node_or_fallback(&current.id);
    let ending = match node.ending.as_ref() {
        Some(ending) => ending,
        None => {
            error!(
                key = "ui.ending.node_missing",
                id = %current.id,
                "Node has no ending data, returning early"
            );
            return;
        }
    };

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
                            font_size: GAME_TEXT_FONT_SIZE,
                            color: TEXT_NORMAL_COLOR,
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        format!("ルートID: {}", current.id),
                        TextStyle {
                            font: font.0.clone(),
                            font_size: CHOICE_FONT_SIZE,
                            color: Color::srgb(0.8, 0.8, 0.8),
                        },
                    ));

                    parent.spawn(TextBundle::from_section(
                        &ending.tag,
                        TextStyle {
                            font: font.0.clone(),
                            font_size: 32.0, // エンディング名は特別なので固有のサイズを保持
                            color: Color::srgb(1.0, 0.8, 0.0),
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0), // Restartボタンのみ異なる幅
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
                        create_button_text_style(font.0.clone(), RESTART_FONT_SIZE),
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
    (Changed<Interaction>, With<Button>, Without<Disabled>),
>;

pub fn button_interaction_system(mut interaction_query: ButtonInteractionQuery) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = BUTTON_PRESSED_COLOR.into();
            }
            Interaction::Hovered => {
                *color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_NORMAL_COLOR.into();
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
