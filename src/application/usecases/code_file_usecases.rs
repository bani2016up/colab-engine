use crate::application::dto::code_file::{
    CodeFileResponse, CreateCodeFileRequest, UpdateCodeRequest, ViewportRequest,
};
use crate::application::errors::ApplicationError;
use crate::application::repositories::code_file_repository::CodeFileRepository;
use crate::domain::code_file::CodeFile;
use crate::domain::traits::dyn_file::{DynemicFileCreateDelete, DynemicFileRead, DynemicFileWrite};
use crate::infrastructure::mmap_file_sys::MmapFileSystemSource;
use std::path::PathBuf;
use uuid::Uuid;

pub trait CodeFileUsecases: Send + Sync {
    fn create_code_file(
        &mut self,
        request: CreateCodeFileRequest,
    ) -> Result<CodeFileResponse, ApplicationError>;
    fn update_code_file(&mut self, request: UpdateCodeRequest) -> Result<(), ApplicationError>;
    fn get_code_file(&self, file_id: Uuid) -> Result<CodeFileResponse, ApplicationError>;
    fn delete_code_file(&mut self, file_id: Uuid) -> Result<(), ApplicationError>;
}

pub struct CodeFileUsecasesImpl {
    pub repository: Box<dyn CodeFileRepository<MmapFileSystemSource>>,
}

impl CodeFileUsecasesImpl {
    pub fn new(repository: Box<dyn CodeFileRepository<MmapFileSystemSource>>) -> Self {
        Self { repository }
    }
}

impl CodeFileUsecases for CodeFileUsecasesImpl {
    fn create_code_file(
        &mut self,
        request: CreateCodeFileRequest,
    ) -> Result<CodeFileResponse, ApplicationError> {
        let file_path = PathBuf::from(format!("/tmp/{}", request.name));

        let temp_source = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        temp_source
            .create_file()
            .map_err(|e| ApplicationError::IoError(e))?;

        let file_sys_source = MmapFileSystemSource::new(file_path)
            .map_err(|e| ApplicationError::IoError(e))?;

        let id = Uuid::new_v4();
        let code_file = CodeFile::new(id, request.name.clone(), file_sys_source.clone());

        let code_file = self.repository.save(code_file)?;
        let code = code_file.source.get_content();

        Ok(CodeFileResponse {
            id: code_file.id(),
            name: request.name,
            viewport: ViewportRequest {
                start_index: 0,
                end_index: code.len() as u64,
                content: code,
            },
        })
    }

    fn update_code_file(&mut self, request: UpdateCodeRequest) -> Result<(), ApplicationError> {
        let mut code_file = self.repository.find_by_id(request.id)?;

        code_file.source.set_slice(
            request.start as usize,
            request.end as usize,
            request.content,
        );

        self.repository.update(code_file)?;

        Ok(())
    }

    fn get_code_file(&self, file_id: Uuid) -> Result<CodeFileResponse, ApplicationError> {
        let code_file = self.repository.find_by_id(file_id)?;
        let code = code_file.source.get_content();
        Ok(CodeFileResponse {
            id: code_file.id(),
            name: code_file.name.clone(),
            viewport: ViewportRequest {
                start_index: 0,
                end_index: code.len() as u64,
                content: code,
            },
        })
    }

    fn delete_code_file(&mut self, file_id: Uuid) -> Result<(), ApplicationError> {
        let code_file = self.repository.find_by_id(file_id)?;

        code_file
            .source
            .delete_file()
            .map_err(|e| ApplicationError::IoError(e))?;

        self.repository.delete(file_id)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::repositories::code_file_repository::CodeFileRepository;
    use crate::domain::code_file::CodeFile;
    use crate::infrastructure::mmap_file_sys::MmapFileSystemSource;
    use crate::infrastructure::persistence::in_memory_repository::InMemoryCodeFileRepository;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use tempfile::TempDir;

    type MockCodeFileRepository = InMemoryCodeFileRepository<MmapFileSystemSource>;

    #[test]
    fn test_create_code_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let request = CreateCodeFileRequest {
            name: "test_file.txt".to_string(),
        };

        let result = usecases.create_code_file(request);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.name, "test_file.txt");
        assert_eq!(response.viewport.start_index, 0);
        assert_eq!(response.viewport.content, "");
    }

    #[test]
    fn test_get_code_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let create_request = CreateCodeFileRequest {
            name: "test_file.txt".to_string(),
        };

        let created = usecases.create_code_file(create_request).unwrap();
        let result = usecases.get_code_file(created.id);

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, created.id);
        assert_eq!(response.name, "test_file.txt");
    }

    #[test]
    fn test_get_code_file_not_found() {
        let repository = Box::new(MockCodeFileRepository::new());
        let usecases = CodeFileUsecasesImpl::new(repository);

        let non_existent_id = Uuid::new_v4();
        let result = usecases.get_code_file(non_existent_id);

        assert!(result.is_err());
        match result {
            Err(ApplicationError::FileNotFound(_)) => {},
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_update_code_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let create_request = CreateCodeFileRequest {
            name: "test_file.txt".to_string(),
        };

        let created = usecases.create_code_file(create_request).unwrap();

        let update_request = UpdateCodeRequest {
            id: created.id,
            start: 0,
            end: 0,
            content: "Hello, World!".to_string(),
        };

        let result = usecases.update_code_file(update_request);
        assert!(result.is_ok());

        let updated_file = usecases.get_code_file(created.id).unwrap();
        assert_eq!(updated_file.viewport.content, "Hello, World!");
    }

    #[test]
    fn test_update_code_file_partial() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let create_request = CreateCodeFileRequest {
            name: "test_file.txt".to_string(),
        };

        let created = usecases.create_code_file(create_request).unwrap();

        let initial_update = UpdateCodeRequest {
            id: created.id,
            start: 0,
            end: 0,
            content: "Hello, World!".to_string(),
        };
        usecases.update_code_file(initial_update).unwrap();

        let partial_update = UpdateCodeRequest {
            id: created.id,
            start: 0,
            end: 5,
            content: "Goodbye".to_string(),
        };

        let result = usecases.update_code_file(partial_update);
        assert!(result.is_ok());

        let updated_file = usecases.get_code_file(created.id).unwrap();
        assert_eq!(updated_file.viewport.content, "Goodbye, World!");
    }

    #[test]
    fn test_update_code_file_not_found() {
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let update_request = UpdateCodeRequest {
            id: Uuid::new_v4(),
            start: 0,
            end: 0,
            content: "Test".to_string(),
        };

        let result = usecases.update_code_file(update_request);
        assert!(result.is_err());
        match result {
            Err(ApplicationError::FileNotFound(_)) => {},
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_delete_code_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let create_request = CreateCodeFileRequest {
            name: "test_file.txt".to_string(),
        };

        let created = usecases.create_code_file(create_request).unwrap();
        let result = usecases.delete_code_file(created.id);

        assert!(result.is_ok());

        let get_result = usecases.get_code_file(created.id);
        assert!(get_result.is_err());
        match get_result {
            Err(ApplicationError::FileNotFound(_)) => {},
            _ => panic!("Expected NotFound error after deletion"),
        }
    }

    #[test]
    fn test_delete_code_file_not_found() {
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let non_existent_id = Uuid::new_v4();
        let result = usecases.delete_code_file(non_existent_id);

        assert!(result.is_err());
        match result {
            Err(ApplicationError::FileNotFound(_)) => {},
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_multiple_files_isolation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let repository = Box::new(MockCodeFileRepository::new());
        let mut usecases = CodeFileUsecasesImpl::new(repository);

        let file1 = usecases
            .create_code_file(CreateCodeFileRequest {
                name: "file1.txt".to_string(),
            })
            .unwrap();

        let file2 = usecases
            .create_code_file(CreateCodeFileRequest {
                name: "file2.txt".to_string(),
            })
            .unwrap();

        usecases
            .update_code_file(UpdateCodeRequest {
                id: file1.id,
                start: 0,
                end: 0,
                content: "Content 1".to_string(),
            })
            .unwrap();

        usecases
            .update_code_file(UpdateCodeRequest {
                id: file2.id,
                start: 0,
                end: 0,
                content: "Content 2".to_string(),
            })
            .unwrap();

        let retrieved_file1 = usecases.get_code_file(file1.id).unwrap();
        let retrieved_file2 = usecases.get_code_file(file2.id).unwrap();

        assert_eq!(retrieved_file1.viewport.content, "Content 1");
        assert_eq!(retrieved_file2.viewport.content, "Content 2");
    }
}
