use uuid::Uuid;
use crate::application::dto::code_file::{CodeFileResponse, ViewportRequest, UpdateCodeRequest, CreateCodeFileRequest};
use crate::application::errors::ApplicationError;
use crate::domain::code_file::CodeFile;
use crate::application::repositories::code_file_repository::CodeFileRepository;
use crate::infrastructure::mmap_file_sys::MmapFileSystemSource;

pub trait CodeFileUsecases: Send + Sync {
    fn create_code_file(&self, request: CreateCodeFileRequest) -> Result<(), ApplicationError>;
    fn update_code_file(&self, request: UpdateCodeRequest) -> Result<(), ApplicationError>;
    fn get_code_file(&self, file_id: Uuid) -> Result<CodeFileResponse, ApplicationError>;
    fn delete_code_file(&self, file_id: Uuid) -> Result<(), ApplicationError>;
}

pub struct CodeFileUsecasesImpl {
    pub repository: Box<dyn CodeFileRepository<CodeFile<MmapFileSystemSource>>>,
}


// impl CodeFileUsecases for CodeFileUsecasesImpl {
//     fn create_code_file(&self, request: CreateCodeFileRequest) -> Result<(), ApplicationError> {
//         let file_sys = MmapFileSystemSource::new(
//             format!("./tmp/{}", request.name)
//         );
//         self.repository.save(
//             CodeFile::new(
//                     Uuid::new_v4(),
//                     request.name,
//                     file_sys

//             )
//         )
//     }
// }
