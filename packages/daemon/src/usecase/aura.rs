#![allow(dead_code)]

use waio_shared::entity::{Aura, AuraError, AuraType};

pub trait AuraRepository {
    fn get(&self, id: &str) -> Result<Aura, AuraError>;
    fn save(&self, aura: &Aura) -> Result<(), AuraError>;
    fn delete(&self, id: &str) -> Result<(), AuraError>;
    fn list(&self) -> Result<Vec<Aura>, AuraError>;
}

pub struct AuraUseCase<R: AuraRepository> {
    pub repo: R,
}

impl<R: AuraRepository> AuraUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn create_aura(
        &self,
        name: String,
        slint_code: String,
        aura_type: AuraType,
    ) -> Result<Aura, AuraError> {
        let id = uuid::Uuid::new_v4().to_string();
        let aura = Aura::new(id, name, aura_type, slint_code);
        aura.validate()?;
        self.repo.save(&aura)?;
        Ok(aura)
    }

    pub fn get_aura(&self, id: &str) -> Result<Aura, AuraError> {
        self.repo.get(id)
    }

    pub fn list_auras(&self) -> Result<Vec<Aura>, AuraError> {
        self.repo.list()
    }

    pub fn delete_aura(&self, id: &str) -> Result<(), AuraError> {
        self.repo.delete(id)
    }
}

pub fn create_aura_without_repo(
    name: String,
    slint_code: String,
    aura_type: AuraType,
) -> Result<Aura, AuraError> {
    let id = uuid::Uuid::new_v4().to_string();
    let aura = Aura::new(id, name, aura_type, slint_code);
    aura.validate()?;
    Ok(aura)
}
