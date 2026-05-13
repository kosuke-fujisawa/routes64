/// セキュリティ機能のテスト
/// パストラバーサル攻撃や不正なファイルアクセスの防御を検証
use routes64::security::*;

#[cfg(test)]
mod security_tests {
    use super::*;

    /// パストラバーサル攻撃の検出と防御
    #[test]
    fn path_traversal_attacks_are_blocked() {
        // 典型的なパストラバーサル攻撃パターン
        let malicious_paths = [
            "../../../etc/passwd",
            "..\\..\\..\\Windows\\System32\\config",
            "images/../../../secret.txt",
            "assets/../../../.ssh/id_rsa",
            "fonts/../../etc/shadow",
        ];

        for malicious_path in &malicious_paths {
            assert!(
                !is_safe_asset_path(malicious_path),
                "Path '{}' should be rejected as unsafe",
                malicious_path
            );

            // secure_asset_pathがフォールバックを返すことを確認
            let safe_result = secure_asset_path(malicious_path, "fallback.png");
            assert_eq!(
                safe_result, "fallback.png",
                "Malicious path '{}' should fall back to default",
                malicious_path
            );
        }
    }

    /// 絶対パスの検出と防御
    #[test]
    fn absolute_paths_are_rejected() {
        let absolute_paths = [
            "/etc/passwd",
            "/usr/local/bin/exploit",
            "C:\\Windows\\System32\\cmd.exe",
            "D:\\secrets\\data.txt",
            "/var/log/messages",
        ];

        for abs_path in &absolute_paths {
            assert!(
                !is_safe_asset_path(abs_path),
                "Absolute path '{}' should be rejected",
                abs_path
            );
        }
    }

    /// 正当なアセットパスが許可されることを確認
    #[test]
    fn legitimate_asset_paths_are_allowed() {
        let legitimate_paths = [
            "images/bg01.png",
            "fonts/NotoSansJP-Regular.ttf",
            "audio/rain.ogg",
            "data/scenario.json",
            "textures/ui/button.png",
            "subdir/file.asset",
        ];

        for legit_path in &legitimate_paths {
            assert!(
                is_safe_asset_path(legit_path),
                "Legitimate path '{}' should be allowed",
                legit_path
            );

            // secure_asset_pathが元のパスを返すことを確認
            let safe_result = secure_asset_path(legit_path, "fallback.png");
            assert_eq!(
                safe_result, *legit_path,
                "Legitimate path '{}' should be returned as-is",
                legit_path
            );
        }
    }

    /// 空文字列やnull的な入力の処理
    #[test]
    fn empty_and_null_inputs_are_handled() {
        assert!(!is_safe_asset_path(""), "Empty string should be rejected");

        let empty_result = secure_asset_path("", "default.asset");
        assert_eq!(
            empty_result, "default.asset",
            "Empty path should use fallback"
        );
    }

    /// エッジケースとして特殊文字を含むパスのテスト
    #[test]
    fn special_characters_in_paths() {
        // 通常は問題ないが、念のため検証する特殊文字
        let special_paths = [
            "images/bg-01.png",      // ハイフン
            "fonts/font_name.ttf",   // アンダースコア
            "audio/rain (1).ogg",    // 括弧とスペース
            "data/scenario.en.json", // ドット複数
        ];

        for special_path in &special_paths {
            // これらは正当なパスとして扱われるべき
            assert!(
                is_safe_asset_path(special_path),
                "Path with special characters '{}' should be allowed",
                special_path
            );
        }
    }
}
