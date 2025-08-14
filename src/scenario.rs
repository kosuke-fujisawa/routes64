use anyhow::{Context, Result};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct Scenario {
    pub meta: Meta,
    pub nodes: Vec<Node>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Meta {
    pub title: String,
    pub depth: usize,
    pub default_background: String,
    #[allow(dead_code)]
    pub rain_bgm: String,
    #[allow(dead_code)]
    pub font: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Node {
    pub id: String,
    pub text: String,
    pub bg: Option<String>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    pub ending: Option<Ending>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Choice {
    pub label: String,
    pub to: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Ending {
    pub tag: String,
}

#[derive(Resource, Debug)]
pub struct ScenarioData {
    pub scenario: Scenario,
    pub nodes: HashMap<String, Node>,
}

#[derive(Resource, Debug, Clone)]
pub struct Current {
    pub id: String,
    pub depth: usize,
    pub trail: Vec<String>,
}

impl Default for Current {
    fn default() -> Self {
        Self {
            id: "R".to_string(),
            depth: 0,
            trail: vec!["R".to_string()],
        }
    }
}

impl ScenarioData {
    pub fn load_from_json(json_content: &str) -> Result<Self> {
        let scenario: Scenario =
            serde_json::from_str(json_content).context("Failed to parse scenario JSON")?;

        let mut nodes = HashMap::new();
        for node in &scenario.nodes {
            nodes.insert(node.id.clone(), node.clone());
        }

        let scenario_data = Self { scenario, nodes };
        scenario_data.validate()?;

        Ok(scenario_data)
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_references()?;
        self.validate_endings();
        Ok(())
    }

    fn validate_references(&self) -> Result<()> {
        for node in &self.scenario.nodes {
            for choice in &node.choices {
                if !self.nodes.contains_key(&choice.to) {
                    return Err(anyhow::anyhow!(
                        "Invalid reference: Node '{}' references non-existent node '{}'",
                        node.id,
                        choice.to
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_endings(&self) {
        let expected_endings = 2_usize.pow(self.scenario.meta.depth as u32);
        let mut actual_endings = 0;
        let mut missing_endings = Vec::new();

        for ending_id in self.generate_ending_ids() {
            if let Some(node) = self.nodes.get(&ending_id) {
                if node.ending.is_some() {
                    actual_endings += 1;
                } else {
                    missing_endings.push(ending_id);
                }
            } else {
                missing_endings.push(ending_id);
            }
        }

        if actual_endings != expected_endings {
            warn!(
                "Expected {} endings but found {}. Missing endings: {:?}",
                expected_endings, actual_endings, missing_endings
            );
        }
    }

    fn generate_ending_ids(&self) -> Vec<String> {
        let depth = self.scenario.meta.depth;
        let mut ending_ids = Vec::new();

        for i in 0..(2_usize.pow(depth as u32)) {
            let mut id = "R".to_string();
            for bit in (0..depth).rev() {
                id.push_str(&((i >> bit) & 1).to_string());
            }
            ending_ids.push(id);
        }

        ending_ids
    }

    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// ノードが存在しない場合は警告を出してroot('R')にフォールバックする
    pub fn get_node_or_fallback(&self, id: &str) -> &Node {
        if let Some(node) = self.nodes.get(id) {
            node
        } else {
            warn!(
                key = "scenario.node_missing",
                id = %id,
                "Node not found, falling back to root 'R'"
            );
            // root('R')が存在しない場合は最初のノードを返す（設計上の問題を回避）
            if let Some(root_node) = self.nodes.get("R") {
                root_node
            } else {
                error!(
                    key = "scenario.root_missing",
                    "Root node 'R' missing, using first available node"
                );
                // 最後の手段：最初に見つかったノードを返す
                self.scenario.nodes.first()
                    .unwrap_or_else(|| {
                        // これは設計上起こり得ないが、空のノードリストの場合の処理
                        panic!("No nodes available in scenario - this indicates a critical configuration error")
                    })
            }
        }
    }

    pub fn transition(&self, current: &Current, choice_index: usize) -> Result<Current> {
        let node = self
            .get_node(&current.id)
            .ok_or_else(|| anyhow::anyhow!("Node not found: {}", current.id))?;

        if choice_index >= node.choices.len() {
            return Err(anyhow::anyhow!(
                "Invalid choice index {} for node {}",
                choice_index,
                current.id
            ));
        }

        let next_id = &node.choices[choice_index].to;
        let new_depth = next_id.len() - 1; // "R"を除く桁数
        let mut new_trail = current.trail.clone();
        new_trail.push(next_id.clone());

        Ok(Current {
            id: next_id.clone(),
            depth: new_depth,
            trail: new_trail,
        })
    }

    pub fn is_ending(&self, current: &Current) -> bool {
        if current.depth == self.scenario.meta.depth {
            if let Some(node) = self.get_node(&current.id) {
                return node.ending.is_some();
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_scenario_json() -> &'static str {
        r#"{
          "meta": {
            "title": "64ルート",
            "depth": 2,
            "default_background": "images/bg01.png",
            "rain_bgm": "audio/rain.ogg",
            "font": "fonts/NotoSansJP-Regular.ttf"
          },
          "nodes": [
            {
              "id": "R",
              "text": "雨。窓に当たる音だけが続く。",
              "bg": "images/bg01.png",
              "choices": [
                {"label": "家を出る", "to": "R1"},
                {"label": "今日は出ない", "to": "R0"}
              ]
            },
            {
              "id": "R1",
              "text": "鍵を手に取る。外は本降りだ。",
              "bg": "images/bg02.png",
              "choices": [
                {"label": "傘をさす", "to": "R11"},
                {"label": "走る", "to": "R10"}
              ]
            },
            {
              "id": "R0",
              "text": "今日は止めておこう。",
              "choices": [
                {"label": "読書する", "to": "R01"},
                {"label": "寝る", "to": "R00"}
              ]
            },
            {
              "id": "R11",
              "text": "傘をさして外に出る。",
              "ending": {"tag": "傘END"}
            },
            {
              "id": "R10",
              "text": "走って外に出る。",
              "ending": {"tag": "走りEND"}
            },
            {
              "id": "R01",
              "text": "本を読んで過ごす。",
              "ending": {"tag": "読書END"}
            },
            {
              "id": "R00",
              "text": "寝て過ごす。",
              "ending": {"tag": "睡眠END"}
            }
          ]
        }"#
    }

    #[test]
    fn test_load_scenario() {
        let scenario_data = ScenarioData::load_from_json(sample_scenario_json()).unwrap();
        assert_eq!(scenario_data.scenario.meta.title, "64ルート");
        assert_eq!(scenario_data.scenario.meta.depth, 2);
        assert_eq!(scenario_data.nodes.len(), 7);
    }

    #[test]
    fn test_transition() {
        let scenario_data = ScenarioData::load_from_json(sample_scenario_json()).unwrap();
        let current = Current::default();

        let next = scenario_data.transition(&current, 0).unwrap();
        assert_eq!(next.id, "R1");
        assert_eq!(next.depth, 1);
        assert_eq!(next.trail, vec!["R", "R1"]);
    }

    #[test]
    fn test_ending_detection() {
        let scenario_data = ScenarioData::load_from_json(sample_scenario_json()).unwrap();
        let ending_current = Current {
            id: "R11".to_string(),
            depth: 2,
            trail: vec!["R".to_string(), "R1".to_string(), "R11".to_string()],
        };

        assert!(scenario_data.is_ending(&ending_current));
    }

    #[test]
    fn test_invalid_reference() {
        let invalid_json = r#"{
          "meta": {"title": "Test", "depth": 1, "default_background": "", "rain_bgm": "", "font": ""},
          "nodes": [
            {
              "id": "R",
              "text": "test",
              "choices": [{"label": "test", "to": "INVALID"}]
            }
          ]
        }"#;

        let result = ScenarioData::load_from_json(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_node_or_fallback() {
        let scenario_data = ScenarioData::load_from_json(sample_scenario_json()).unwrap();

        // 存在するノード
        let existing_node = scenario_data.get_node_or_fallback("R1");
        assert_eq!(existing_node.id, "R1");

        // 存在しないノード（root 'R' にフォールバックする）
        let fallback_node = scenario_data.get_node_or_fallback("NONEXISTENT");
        assert_eq!(fallback_node.id, "R");
    }
}
