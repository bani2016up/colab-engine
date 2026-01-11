use crate::application::errors::ApplicationError;
use crate::domain::code_file::CodeFile;
use crate::domain::traits::dyn_file::{DynemicFileCreateDelete, DynemicFileRead, DynemicFileWrite};
use uuid::Uuid;

pub trait CodeFileRepository<FileSource>: Send + Sync
where
    FileSource: DynemicFileRead + DynemicFileWrite + DynemicFileCreateDelete,
{
    fn save(&mut self, file: CodeFile<FileSource>) -> Result<CodeFile<FileSource>, ApplicationError>;
    fn find_by_id(&self, id: Uuid) -> Result<CodeFile<FileSource>, ApplicationError>;
    fn update(&mut self, file: CodeFile<FileSource>) -> Result<(), ApplicationError>;
    fn delete(&mut self, id: Uuid) -> Result<(), ApplicationError>;
    fn list(&self) -> Result<Vec<CodeFile<FileSource>>, ApplicationError>;
}
