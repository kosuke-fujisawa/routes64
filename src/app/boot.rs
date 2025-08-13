use crate::app::constants::{SCENARIO_PATH, DEFAULT_FONT};
use crate::scenario::{Current, ScenarioData};
use crate::states::AppState;
use crate::ui::GameFont;
use bevy::prelude::*;

#[cfg(feature = "rain_bgm")]
use crate::app::constants::RAIN_AUDIO_PATH;
#[cfg(feature = "rain_bgm")]
use crate::audio::RainAudioHandle;

#[derive(Resource, Default)]
pub struct LoadingResources {
    pub scenario_json: Option<String>,
    pub font_handle: Option<Handle<Font>>,
    #[cfg(feature = "rain_bgm")]
    pub rain_handle: Option<Handle<AudioSource>>,
}

#[derive(Resource, Default)]
pub struct ResourceReadiness {
    pub scenario_loaded: bool,
    pub font_loaded: bool,
    #[cfg(feature = "rain_bgm")]
    pub rain_loaded: bool,
}

impl ResourceReadiness {
    pub fn all_ready(&self) -> bool {
        let base_ready = self.scenario_loaded && self.font_loaded;
        #[cfg(feature = "rain_bgm")]
        {
            base_ready && self.rain_loaded
        }
        #[cfg(not(feature = "rain_bgm"))]
        {
            base_ready
        }
    }
}

/// Boot ステートでリソースのロードを開始
pub fn start_resource_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("Starting resource loading...");
    
    // シナリオファイルを同期的に読み込み（しばらくは既存のアプローチを維持）
    let scenario_json = match std::fs::read_to_string(SCENARIO_PATH) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read {}: {}", SCENARIO_PATH, e);
            return;
        }
    };
    
    // フォントファイルを非同期ロード  
    let font_handle: Handle<Font> = asset_server.load(DEFAULT_FONT);
    
    // 雨音ファイルを非同期ロード（rain_bgm feature が有効の場合）
    #[cfg(feature = "rain_bgm")]
    let rain_handle: Handle<AudioSource> = asset_server.load(RAIN_AUDIO_PATH);
    
    commands.insert_resource(LoadingResources {
        scenario_json: Some(scenario_json),
        font_handle: Some(font_handle.clone()),
        #[cfg(feature = "rain_bgm")]
        rain_handle: Some(rain_handle.clone()),
    });
    
    commands.insert_resource(ResourceReadiness::default());
    commands.insert_resource(GameFont(font_handle));
    
    #[cfg(feature = "rain_bgm")]
    commands.insert_resource(RainAudioHandle(rain_handle));
}

/// リソースの準備状況をチェックし、準備完了時にTitleステートに遷移
pub fn check_resources_loaded(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    loading_resources: Res<LoadingResources>,
    mut resource_readiness: ResMut<ResourceReadiness>,
    asset_server: Res<AssetServer>,
) {
    // シナリオファイルの読み込み状況をチェック
    if !resource_readiness.scenario_loaded {
        if let Some(json_content) = &loading_resources.scenario_json {
            match ScenarioData::load_from_json(json_content) {
                Ok(scenario_data) => {
                    commands.insert_resource(scenario_data);
                    commands.insert_resource(Current::default());
                    resource_readiness.scenario_loaded = true;
                    info!("Scenario loaded successfully");
                }
                Err(e) => {
                    error!("Failed to parse scenario: {}", e);
                    return;
                }
            }
        }
    }
    
    // フォントの読み込み状況をチェック
    if !resource_readiness.font_loaded {
        if let Some(font_handle) = &loading_resources.font_handle {
            match asset_server.load_state(font_handle) {
                bevy::asset::LoadState::Loaded => {
                    resource_readiness.font_loaded = true;
                    info!("Font loaded successfully");
                }
                bevy::asset::LoadState::Failed(_) => {
                    error!("Failed to load font");
                    return;
                }
                _ => {
                    // まだ読み込み中
                }
            }
        }
    }
    
    // 雨音の読み込み状況をチェック（rain_bgm feature が有効の場合）
    #[cfg(feature = "rain_bgm")]
    if !resource_readiness.rain_loaded {
        if let Some(rain_handle) = &loading_resources.rain_handle {
            match asset_server.load_state(rain_handle) {
                bevy::asset::LoadState::Loaded => {
                    resource_readiness.rain_loaded = true;
                    info!("Rain audio loaded successfully");
                }
                bevy::asset::LoadState::Failed(_) => {
                    error!("Failed to load rain audio");
                    return;
                }
                _ => {
                    // まだ読み込み中
                }
            }
        }
    }
    
    // 全てのリソースが準備完了したらTitleステートに遷移
    if resource_readiness.all_ready() {
        next_state.set(AppState::Title);
        info!("All resources loaded, transitioning to Title");
    }
}