/// アプリケーション全体で使用する定数

/// シナリオファイルのパス
pub const SCENARIO_PATH: &str = "assets/scenario.json";

/// 利用可能なフォント候補（優先順）
pub const FONT_CANDIDATES: &[&str] = &[
    "fonts/NotoSansJP-Regular.ttf",
    "fonts/NotoSansJP-Medium.ttf",
    "fonts/NotoSansJP-Bold.ttf",
];

/// デフォルトフォント
pub const DEFAULT_FONT: &str = FONT_CANDIDATES[0];

/// 雨音ファイルのパス
#[cfg(feature = "rain_bgm")]
pub const RAIN_AUDIO_PATH: &str = "audio/rain.ogg";

/// ゲーム設定定数
pub mod ui {
    use bevy::prelude::Color;
    
    /// ボタンの標準サイズ
    pub const BUTTON_WIDTH: f32 = 200.0;
    pub const BUTTON_HEIGHT: f32 = 50.0;
    
    /// ボタンの色
    pub const BUTTON_NORMAL_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BUTTON_HOVER_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const BUTTON_DISABLED_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
    
    /// テキストの色
    pub const TEXT_NORMAL_COLOR: Color = Color::WHITE;
    pub const TEXT_DISABLED_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
    
    /// フォントサイズ
    pub const TITLE_FONT_SIZE: f32 = 48.0;
    pub const BUTTON_FONT_SIZE: f32 = 24.0;
    pub const GAME_TEXT_FONT_SIZE: f32 = 24.0;
    pub const CHOICE_FONT_SIZE: f32 = 18.0;
    pub const RESTART_FONT_SIZE: f32 = 20.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constants() {
        assert!(!SCENARIO_PATH.is_empty());
        assert!(!FONT_CANDIDATES.is_empty());
        assert_eq!(DEFAULT_FONT, FONT_CANDIDATES[0]);
    }
    
    #[test]
    fn test_ui_constants() {
        assert!(ui::BUTTON_WIDTH > 0.0);
        assert!(ui::BUTTON_HEIGHT > 0.0);
        assert!(ui::TITLE_FONT_SIZE > ui::BUTTON_FONT_SIZE);
    }
}