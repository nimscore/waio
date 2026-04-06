use crate::usecase::aura::AuraRepository;
use std::fs;
use std::path::PathBuf;
use waio_shared::entity::{Aura, AuraError};

pub struct FileAuraRepository {
    base_path: PathBuf,
}

impl FileAuraRepository {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    fn aura_path(&self, id: &str) -> PathBuf {
        self.base_path.join(format!("{}.json", id))
    }
}

impl AuraRepository for FileAuraRepository {
    fn get(&self, id: &str) -> Result<Aura, AuraError> {
        let content = fs::read_to_string(self.aura_path(id))
            .map_err(|e| AuraError::NotFound(e.to_string()))?;
        serde_json::from_str(&content).map_err(|e| AuraError::InvalidSyntax(e.to_string()))
    }

    fn save(&self, aura: &Aura) -> Result<(), AuraError> {
        let content =
            serde_json::to_string(aura).map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        fs::write(self.aura_path(&aura.id), content)
            .map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        Ok(())
    }

    fn delete(&self, id: &str) -> Result<(), AuraError> {
        fs::remove_file(self.aura_path(id)).map_err(|e| AuraError::NotFound(e.to_string()))?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<Aura>, AuraError> {
        let mut auras = Vec::new();
        let entries =
            fs::read_dir(&self.base_path).map_err(|e| AuraError::InvalidSyntax(e.to_string()))?;
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(aura) = serde_json::from_str::<Aura>(&content) {
                        auras.push(aura);
                    }
                }
            }
        }
        Ok(auras)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use waio_shared::entity::{AuraType, LayerAnchor, Size};

    fn make_test_aura(id: &str) -> Aura {
        Aura {
            id: id.to_string(),
            name: "Test".into(),
            aura_type: AuraType::Bar,
            slint_code: "export component X inherits Window {}".into(),
            lua_code: Some("print('test')".into()),
            config: waio_shared::entity::AuraConfig {
                anchor: LayerAnchor::Top,
                size: Size {
                    width: 1920,
                    height: 40,
                },
                exclusive_zone: 40,
                output: None,
            },
            layers: vec![],
            permissions: vec!["timer".into()],
            exec_commands: vec![],
        }
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo = FileAuraRepository::new(temp_dir.path().to_path_buf());
        let aura = make_test_aura("test-1");
        repo.save(&aura).unwrap();
        let loaded = repo.get("test-1").unwrap();
        assert_eq!(loaded.id, "test-1");
        assert_eq!(loaded.name, "Test");
        assert_eq!(loaded.permissions, vec!["timer"]);
    }

    #[test]
    fn test_delete() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo = FileAuraRepository::new(temp_dir.path().to_path_buf());
        let aura = make_test_aura("test-2");
        repo.save(&aura).unwrap();
        repo.delete("test-2").unwrap();
        assert!(repo.get("test-2").is_err());
    }

    #[test]
    fn test_list() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo = FileAuraRepository::new(temp_dir.path().to_path_buf());
        repo.save(&make_test_aura("a")).unwrap();
        repo.save(&make_test_aura("b")).unwrap();
        let auras = repo.list().unwrap();
        assert_eq!(auras.len(), 2);
    }

    #[test]
    fn test_get_not_found() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo = FileAuraRepository::new(temp_dir.path().to_path_buf());
        assert!(matches!(
            repo.get("nonexistent"),
            Err(AuraError::NotFound(_))
        ));
    }
}
