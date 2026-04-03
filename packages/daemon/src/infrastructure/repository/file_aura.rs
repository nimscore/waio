#![allow(dead_code)]

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
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&entry.path()) {
                        if let Ok(aura) = serde_json::from_str::<Aura>(&content) {
                            auras.push(aura);
                        }
                    }
                }
            }
        }
        Ok(auras)
    }
}
