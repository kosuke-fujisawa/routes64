/// アセット読み込み失敗時の動作テスト
/// システムが適切にエラーハンドリングを行うことを検証
use routes64::*;

#[cfg(test)]
mod failure_injection_tests {
    use super::*;

    /// 不正なシナリオファイルをロードした際の適切なエラーハンドリング
    #[test]
    fn scenario_load_error_handles_gracefully() {
        let invalid_json = r#"{
            "meta": {
                "title": "不正なテスト",
                "depth": 1
                // 必須フィールドが不足
            },
            "nodes": []
        }"#;

        let result = ScenarioData::load_from_json(invalid_json);
        assert!(result.is_err(), "Invalid JSON should cause an error");

        // エラーメッセージが適切であることを確認（デバッグ情報として）
        if let Err(e) = result {
            let error_msg = e.to_string();
            // 少なくとも何らかのエラー情報が含まれていることを確認
            assert!(!error_msg.is_empty(), "Error message should not be empty");
        }
    }

    /// 循環参照を含むシナリオの検証
    #[test]
    fn scenario_with_invalid_references_fails_validation() {
        let circular_json = r#"{
            "meta": {
                "title": "循環参照テスト",
                "depth": 1,
                "default_background": "images/bg01.png",
                "rain_bgm": "audio/rain.ogg",
                "font": "fonts/test.ttf"
            },
            "nodes": [
                {
                    "id": "R",
                    "text": "スタート",
                    "choices": [
                        {"label": "存在しないノードへ", "to": "NONEXISTENT"}
                    ]
                }
            ]
        }"#;

        let result = ScenarioData::load_from_json(circular_json);
        assert!(
            result.is_err(),
            "Invalid node references should cause validation error"
        );
    }

    /// get_node_or_fallback の境界条件テスト
    #[test]
    fn get_node_or_fallback_handles_edge_cases() {
        let minimal_scenario = r#"{
            "meta": {
                "title": "最小シナリオ",
                "depth": 1,
                "default_background": "images/bg01.png",
                "rain_bgm": "audio/rain.ogg",
                "font": "fonts/test.ttf"
            },
            "nodes": [
                {
                    "id": "ONLY",
                    "text": "唯一のノード",
                    "choices": []
                }
            ]
        }"#;

        let scenario_data = ScenarioData::load_from_json(minimal_scenario).unwrap();

        // 存在しないノードIDを要求
        let result = scenario_data.get_node_or_fallback("NONEXISTENT");
        assert!(
            result.is_ok(),
            "get_node_or_fallback should handle missing nodes gracefully"
        );

        // フォールバック先も存在しない場合（'R'ノードなし）
        let fallback_node = result.unwrap();
        assert_eq!(
            fallback_node.id, "ONLY",
            "Should fall back to first available node"
        );
    }

    /// 空のシナリオに対するエラーハンドリング
    #[test]
    fn empty_scenario_fails_gracefully() {
        let empty_scenario = r#"{
            "meta": {
                "title": "空シナリオ",
                "depth": 0,
                "default_background": "images/bg01.png",
                "rain_bgm": "audio/rain.ogg",
                "font": "fonts/test.ttf"
            },
            "nodes": []
        }"#;

        let scenario_data = ScenarioData::load_from_json(empty_scenario).unwrap();

        // 空のノードリストに対してget_node_or_fallbackを呼び出し
        let result = scenario_data.get_node_or_fallback("ANY");
        assert!(result.is_err(), "Empty node list should cause error");
    }
}
