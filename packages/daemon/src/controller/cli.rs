use crate::infrastructure::repository::FileAuraRepository;
use crate::usecase::aura::AuraUseCase;
use anyhow::Result;

pub struct CliController {
    aura_use_case: AuraUseCase<FileAuraRepository>,
}

impl CliController {
    pub fn new(uc: AuraUseCase<FileAuraRepository>) -> Self {
        Self { aura_use_case: uc }
    }

    pub fn create_bar(&self, name: String, slint_code: String) -> Result<String, String> {
        let aura = self
            .aura_use_case
            .create_aura(name, slint_code, waio_shared::entity::AuraType::Bar)
            .map_err(|e| e.to_string())?;
        Ok(format!("Created bar: {}", aura.id))
    }
}
