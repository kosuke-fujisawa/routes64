/// セキュリティ関連の機能
/// パストラバーサル攻撃や不正なファイルアクセスを防ぐ
use bevy::prelude::*;

/// アセットパスのセキュリティチェック
/// - パストラバーサル（..）を防ぐ
/// - ルート外アクセスを防ぐ
pub fn is_safe_asset_path(path: &str) -> bool {
    // 空文字列を拒否
    if path.is_empty() {
        warn!(
            key = "security.path.empty",
            path = %path,
            "Empty path rejected"
        );
        return false;
    }

    // パストラバーサル（..）を含む場合は拒否
    if path.contains("..") {
        warn!(
            key = "security.path.traversal",
            path = %path,
            "Path traversal attempt blocked"
        );
        return false;
    }

    // 絶対パス（/で始まる）を拒否（相対パスのみ許可）
    if path.starts_with('/') {
        warn!(
            key = "security.path.absolute",
            path = %path,
            "Absolute path rejected"
        );
        return false;
    }

    // Windowsドライブ文字を拒否
    if path.len() >= 2 && path.chars().nth(1) == Some(':') {
        warn!(
            key = "security.path.drive",
            path = %path,
            "Drive path rejected"
        );
        return false;
    }

    true
}

/// セキュリティチェックを通したアセットロード
/// 不正なパスの場合はエラーログを出力し、デフォルトパスを返す
pub fn secure_asset_path(path: &str, default_path: &str) -> String {
    if is_safe_asset_path(path) {
        path.to_string()
    } else {
        error!(
            key = "security.path.rejected",
            requested_path = %path,
            fallback_path = %default_path,
            "Unsafe path rejected, using fallback"
        );
        default_path.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_paths() {
        assert!(is_safe_asset_path("images/bg01.png"));
        assert!(is_safe_asset_path("fonts/NotoSansJP-Regular.ttf"));
        assert!(is_safe_asset_path("audio/rain.ogg"));
        assert!(is_safe_asset_path("subdir/file.json"));
    }

    #[test]
    fn test_unsafe_paths() {
        assert!(!is_safe_asset_path("../../../etc/passwd"));
        assert!(!is_safe_asset_path("images/../../../secret.txt"));
        assert!(!is_safe_asset_path("/etc/passwd"));
        assert!(!is_safe_asset_path("C:\\Windows\\system32\\config"));
        assert!(!is_safe_asset_path(""));
    }

    #[test]
    fn test_secure_asset_path() {
        assert_eq!(
            secure_asset_path("images/bg01.png", "default.png"),
            "images/bg01.png".to_string()
        );
        assert_eq!(
            secure_asset_path("../malicious.txt", "default.png"),
            "default.png".to_string()
        );
        assert_eq!(
            secure_asset_path("/etc/passwd", "default.png"),
            "default.png".to_string()
        );
    }
}
