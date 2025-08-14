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
    fn test_constants_validity() {
        // 定数の内容をチェック（動的値ではないため、基本的な構造のみテスト）

        // フォント候補配列が設定されていることを確認

        // フォント候補配列の構造を確認
        assert_eq!(FONT_CANDIDATES.len(), 3);

        // デフォルトフォントが最初の候補と一致することを確認
        assert_eq!(DEFAULT_FONT, FONT_CANDIDATES[0]);
    }

    #[test]
    fn test_ui_constants_values() {
        // UI定数の具体的な値を確認（将来の変更検知のため）
        assert_eq!(ui::BUTTON_WIDTH, 200.0);
        assert_eq!(ui::BUTTON_HEIGHT, 50.0);
        assert_eq!(ui::TITLE_FONT_SIZE, 48.0);
        assert_eq!(ui::BUTTON_FONT_SIZE, 24.0);

        // 色定数が設定されていることを確認
        assert_ne!(ui::BUTTON_NORMAL_COLOR, ui::BUTTON_HOVER_COLOR);
        assert_ne!(ui::TEXT_NORMAL_COLOR, ui::TEXT_DISABLED_COLOR);
    }
}
