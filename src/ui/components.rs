use crate::app::constants::ui::*;
use bevy::prelude::*;

/// ボタンが無効化されていることを示すコンポーネント
#[derive(Component)]
pub struct Disabled;

/// 統一されたゲームボタンを作成する関数
pub fn create_game_button() -> ButtonBundle {
    create_game_button_with_color(BUTTON_NORMAL_COLOR)
}

/// 指定された色でゲームボタンを作成する関数
pub fn create_game_button_with_color(color: Color) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

/// ゲームボタンのテキストスタイルを作成する関数
pub fn create_button_text_style(font: Handle<Font>, size: f32) -> TextStyle {
    TextStyle {
        font,
        font_size: size,
        color: TEXT_NORMAL_COLOR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_component() {
        let disabled = Disabled;
        // Component trait が実装されていることを確認
        // 実際のテストでは Entity に attach してテストする
        std::mem::drop(disabled);
    }

    #[test]
    fn test_create_game_button() {
        let button = create_game_button();

        // 期待される設定値を検証
        assert_eq!(button.style.width, Val::Px(200.0));
        assert_eq!(button.style.height, Val::Px(50.0));
        assert_eq!(button.style.justify_content, JustifyContent::Center);
        assert_eq!(button.style.align_items, AlignItems::Center);
    }

    #[test]
    fn test_create_button_text_style() {
        let font_handle: Handle<Font> = Handle::weak_from_u128(12345);
        let text_style = create_button_text_style(font_handle.clone(), 24.0);

        assert_eq!(text_style.font, font_handle);
        assert_eq!(text_style.font_size, 24.0);
        assert_eq!(text_style.color, Color::WHITE);
    }
}
