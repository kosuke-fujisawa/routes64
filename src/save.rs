use crate::scenario::Current;
use anyhow::{Context, Result};
use bevy::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveData {
    pub version: u8,
    pub current: String,
    pub depth: usize,
    pub trail: Vec<String>,
}

#[derive(Resource)]
pub struct SaveManager {
    save_path: PathBuf,
    disabled: bool,
}

impl SaveManager {
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "routes64", "routes64")
            .context("Failed to get project directories")?;

        let data_dir = project_dirs.data_local_dir();
        fs::create_dir_all(data_dir)
            .with_context(|| format!("Failed to create data directory: {data_dir:?}"))?;

        let save_path = data_dir.join("save.json");

        Ok(Self {
            save_path,
            disabled: false,
        })
    }

    /// セーブ機能が無効化されたSaveManagerを作成
    pub fn new_disabled() -> Self {
        Self {
            save_path: PathBuf::new(), // 無効なパス
            disabled: true,
        }
    }

    pub fn save(&self, current: &Current) -> Result<()> {
        if self.disabled {
            debug!("Save disabled, skipping save operation");
            return Ok(());
        }

        let save_data = SaveData {
            version: 1,
            current: current.id.clone(),
            depth: current.depth,
            trail: current.trail.clone(),
        };

        let json =
            serde_json::to_string_pretty(&save_data).context("Failed to serialize save data")?;

        fs::write(&self.save_path, json).with_context(|| {
            format!(
                "Failed to write save file: {save_path:?}",
                save_path = self.save_path
            )
        })?;

        info!("Game saved to {:?}", self.save_path);
        Ok(())
    }

    pub fn load(&self) -> Result<Option<Current>> {
        if self.disabled {
            debug!("Save disabled, no save data available");
            return Ok(None);
        }

        if !self.save_path.exists() {
            return Ok(None);
        }

        let json = fs::read_to_string(&self.save_path).with_context(|| {
            format!(
                "Failed to read save file: {save_path:?}",
                save_path = self.save_path
            )
        })?;

        let save_data: SaveData =
            serde_json::from_str(&json).context("Failed to deserialize save data")?;

        if save_data.version != 1 {
            warn!(
                "Save file version mismatch. Expected: 1, Found: {}. Save will be ignored.",
                save_data.version
            );
            return Ok(None);
        }

        let current = Current {
            id: save_data.current,
            depth: save_data.depth,
            trail: save_data.trail,
        };

        info!("Game loaded from {:?}", self.save_path);
        Ok(Some(current))
    }

    #[allow(dead_code)]
    pub fn delete(&self) -> Result<()> {
        if self.save_path.exists() {
            fs::remove_file(&self.save_path).with_context(|| {
                format!(
                    "Failed to delete save file: {save_path:?}",
                    save_path = self.save_path
                )
            })?;
            info!("Save file deleted: {:?}", self.save_path);
        }
        Ok(())
    }

    pub fn has_save(&self) -> bool {
        if self.disabled {
            return false;
        }
        self.save_path.exists()
    }
}

pub fn auto_save_system(save_manager: Res<SaveManager>, current: Res<Current>) {
    if current.is_changed() && current.depth > 0 {
        if let Err(e) = save_manager.save(&current) {
            error!("Failed to auto-save: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_save_manager() -> (SaveManager, tempfile::TempDir) {
        let temp_dir = tempdir().unwrap();
        let save_path = temp_dir.path().join("test_save.json");

        let save_manager = SaveManager {
            save_path,
            disabled: false,
        };

        (save_manager, temp_dir)
    }

    #[test]
    fn test_save_and_load() {
        let (save_manager, _temp_dir) = create_test_save_manager();

        let original_current = Current {
            id: "R101".to_string(),
            depth: 3,
            trail: vec![
                "R".to_string(),
                "R1".to_string(),
                "R10".to_string(),
                "R101".to_string(),
            ],
        };

        save_manager.save(&original_current).unwrap();
        assert!(save_manager.has_save());

        let loaded_current = save_manager.load().unwrap().unwrap();
        assert_eq!(loaded_current.id, original_current.id);
        assert_eq!(loaded_current.depth, original_current.depth);
        assert_eq!(loaded_current.trail, original_current.trail);
    }

    #[test]
    fn test_load_nonexistent_save() {
        let (save_manager, _temp_dir) = create_test_save_manager();

        let result = save_manager.load().unwrap();
        assert!(result.is_none());
        assert!(!save_manager.has_save());
    }

    #[test]
    fn test_delete_save() {
        let (save_manager, _temp_dir) = create_test_save_manager();
        let current = Current::default();

        save_manager.save(&current).unwrap();
        assert!(save_manager.has_save());

        save_manager.delete().unwrap();
        assert!(!save_manager.has_save());
    }

    #[test]
    fn test_version_mismatch() {
        let (save_manager, _temp_dir) = create_test_save_manager();

        let invalid_save = r#"{"version": 255, "current": "R", "depth": 0, "trail": ["R"]}"#;
        fs::write(&save_manager.save_path, invalid_save).unwrap();

        let result = save_manager.load().unwrap();
        assert!(result.is_none());
    }
}
