use uuid::Uuid;
use crate::domain::code_file::CodeFile;
use crate::domain::traits::dyn_file::{DynemicFileRead, DynemicFileWrite, DynemicFileCreateDelete};
use crate::application::errors::ApplicationError;

pub trait CodeFileRepository<T>: Send + Sync
where
    T: DynemicFileRead + DynemicFileWrite + DynemicFileCreateDelete,
{
    fn save(&mut self, file: CodeFile<T>) -> Result<(), ApplicationError>;
    fn find_by_id(&self, id: Uuid) -> Result<CodeFile<T>, ApplicationError>;
    fn update(&mut self, file: CodeFile<T>) -> Result<(), ApplicationError>;
    fn delete(&mut self, id: Uuid) -> Result<(), ApplicationError>;
    fn list(&self) -> Result<Vec<CodeFile<T>>, ApplicationError>;
}
