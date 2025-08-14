/// シナリオフォールバック機能のテスト（ユニットテスト）  
use routes64::scenario::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_scenario() -> ScenarioData {
        let scenario_json = r#"{
            "meta": {
                "title": "Test",
                "depth": 2,
                "default_background": "images/bg01.png",
                "rain_bgm": "audio/rain.ogg", 
                "font": "fonts/test.ttf"
            },
            "nodes": [
                {
                    "id": "R",
                    "text": "Root node text",
                    "bg": "images/bg01.png",
                    "choices": [
                        {"label": "Choice A", "to": "R1"},
                        {"label": "Choice B", "to": "R0"}
                    ]
                },
                {
                    "id": "R1", 
                    "text": "Node R1 text",
                    "choices": []
                },
                {
                    "id": "R0",
                    "text": "Node R0 text", 
                    "choices": []
                }
            ]
        }"#;

        ScenarioData::load_from_json(scenario_json).unwrap()
    }

    /// 存在するノードIDでは該当ノードが返されることを確認
    #[test]
    fn scenario_get_node_or_fallback_returns_existing_node() {
        let scenario_data = create_test_scenario();

        let node = scenario_data.get_node_or_fallback("R1");
        assert_eq!(node.id, "R1");
        assert_eq!(node.text, "Node R1 text");
    }

    /// 存在しないノードIDではルートノード'R'が返されることを確認
    #[test]
    fn scenario_get_node_or_fallback_returns_root() {
        let scenario_data = create_test_scenario();

        let node = scenario_data.get_node_or_fallback("NONEXISTENT");
        assert_eq!(node.id, "R");
        assert_eq!(node.text, "Root node text");
    }

    /// ルートノード'R'も存在しない場合は最初のノードが返されることを確認
    #[test]
    fn scenario_get_node_or_fallback_returns_first_when_no_root() {
        let scenario_json_without_root = r#"{
            "meta": {
                "title": "Test",
                "depth": 1,
                "default_background": "images/bg01.png",
                "rain_bgm": "audio/rain.ogg",
                "font": "fonts/test.ttf"
            },
            "nodes": [
                {
                    "id": "START",
                    "text": "Start node text",
                    "choices": []
                },
                {
                    "id": "OTHER",
                    "text": "Other node text",
                    "choices": []
                }
            ]
        }"#;

        let scenario_data = ScenarioData::load_from_json(scenario_json_without_root).unwrap();

        let node = scenario_data.get_node_or_fallback("NONEXISTENT");
        // ルート'R'が存在しないので最初のノード'START'が返される
        assert_eq!(node.id, "START");
        assert_eq!(node.text, "Start node text");
    }

    /// get_nodeメソッドは存在しないIDでNoneを返すことを確認
    #[test]
    fn scenario_get_node_returns_none_for_nonexistent() {
        let scenario_data = create_test_scenario();

        let result = scenario_data.get_node("NONEXISTENT");
        assert!(result.is_none());

        let result = scenario_data.get_node("R1");
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "R1");
    }
}
