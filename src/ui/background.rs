/// 背景スプライト管理システム
/// 一意性を保証し、不正なアセットパスを防ぐ
use crate::scenario::ScenarioData;
use crate::security::secure_asset_path;
use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundSprite;

/// 背景スプライトが存在しない場合のみ作成する（一意性保証）
/// 複数存在する場合は警告を出力し、余分なものを削除する
pub fn setup_background_if_needed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario_data: Res<ScenarioData>,
    background_query: Query<Entity, With<BackgroundSprite>>,
) {
    let background_count = background_query.iter().count();

    if background_count == 0 {
        let safe_path = secure_asset_path(
            &scenario_data.scenario.meta.default_background,
            "images/bg01.png",
        );
        let background_handle: Handle<Image> = asset_server.load(safe_path);

        commands.spawn((
            SpriteBundle {
                texture: background_handle,
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            BackgroundSprite,
        ));
        info!("Background sprite created");
    } else if background_count == 1 {
        debug!("Background sprite already exists, skipping creation");
    } else {
        // 複数の背景スプライトが存在する場合は警告を出し、最初の1つ以外を削除
        warn!(
            key = "ui.background.multiple_sprites",
            count = background_count,
            "Multiple background sprites detected, cleaning up"
        );

        let entities: Vec<Entity> = background_query.iter().collect();
        for entity in entities.iter().skip(1) {
            commands.entity(*entity).despawn_recursive();
        }
        info!(
            "Cleaned up {} extra background sprites",
            background_count - 1
        );
    }
}

/// 背景を無条件で作成する（直接呼び出し用）
/// 将来的に背景の強制再作成が必要になった場合のために保持
#[allow(dead_code)]
pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scenario_data: Res<ScenarioData>,
) {
    let safe_path = secure_asset_path(
        &scenario_data.scenario.meta.default_background,
        "images/bg01.png",
    );
    let background_handle: Handle<Image> = asset_server.load(safe_path);

    commands.spawn((
        SpriteBundle {
            texture: background_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        BackgroundSprite,
    ));
}

/// 背景画像の動的更新システム
/// Current リソースが変更された時に背景を適切に切り替える
pub fn update_background(
    mut background_query: Query<&mut Handle<Image>, With<BackgroundSprite>>,
    asset_server: Res<AssetServer>,
    scenario_data: Res<ScenarioData>,
    current: Res<crate::scenario::Current>,
) {
    // Currentが変更された場合のみ処理を実行
    if current.is_changed() {
        if let Ok(mut background_handle) = background_query.get_single_mut() {
            let new_bg = if let Some(node) = scenario_data.get_node(&current.id) {
                node.bg
                    .as_ref()
                    .unwrap_or(&scenario_data.scenario.meta.default_background)
            } else {
                &scenario_data.scenario.meta.default_background
            };

            // セキュリティチェックを通したパスでロード
            let safe_bg =
                secure_asset_path(new_bg, &scenario_data.scenario.meta.default_background);
            *background_handle = asset_server.load(safe_bg);
        }
    }
}
