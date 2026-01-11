use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::domain::code_file::CodeFile;
use crate::domain::traits::dyn_file::{DynemicFileRead, DynemicFileWrite, DynemicFileCreateDelete};
use crate::application::repositories::code_file_repository::CodeFileRepository;
use crate::application::errors::ApplicationError;

pub struct InMemoryCodeFileRepository<T>
where
    T: DynemicFileRead + DynemicFileWrite + DynemicFileCreateDelete,
{
    storage: Arc<RwLock<HashMap<Uuid, CodeFile<T>>>>,
}

impl<T> InMemoryCodeFileRepository<T>
where
    T: DynemicFileRead + DynemicFileWrite + DynemicFileCreateDelete,
{
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl<T> CodeFileRepository<T> for InMemoryCodeFileRepository<T>
where
    T: DynemicFileRead + DynemicFileWrite + DynemicFileCreateDelete + Clone + Send + Sync,
{
    fn save(&mut self, file: CodeFile<T>) -> Result<(), ApplicationError> {
        let mut storage = self.storage.write().unwrap();
        storage.insert(file.id(), file);
        Ok(())
    }

    fn find_by_id(&self, id: Uuid) -> Result<CodeFile<T>, ApplicationError> {
        let storage = self.storage.read().unwrap();
        storage
            .get(&id)
            .cloned()
            .ok_or_else(|| ApplicationError::FileNotFound(id.to_string()))
    }

    fn update(&mut self, file: CodeFile<T>) -> Result<(), ApplicationError> {
        let mut storage = self.storage.write().unwrap();
        if !storage.contains_key(&file.id()) {
            return Err(ApplicationError::FileNotFound(file.id().to_string()));
        }
        storage.insert(file.id(), file);
        Ok(())
    }

    fn delete(&mut self, id: Uuid) -> Result<(), ApplicationError> {
        let mut storage = self.storage.write().unwrap();
        storage
            .remove(&id)
            .ok_or_else(|| ApplicationError::FileNotFound(id.to_string()))?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<CodeFile<T>>, ApplicationError> {
        let storage = self.storage.read().unwrap();
        Ok(storage.values().cloned().collect())
    }
}
