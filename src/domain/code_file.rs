use uuid::Uuid;

use crate::domain::traits::dyn_file::{DynemicFileCreateDelete, DynemicFileRead, DynemicFileWrite};

#[derive(Debug, Clone)]
pub struct CodeFile<FileSource>
where
    FileSource: DynemicFileRead + DynemicFileWrite + DynemicFileCreateDelete,
{
    id: Uuid,
    pub name: String,
    pub source: FileSource,
}

impl<FileSource> CodeFile<FileSource>
where
    FileSource: DynemicFileRead + DynemicFileWrite + DynemicFileCreateDelete,
{
    pub fn new(id: Uuid, name: String, source: FileSource) -> Self {
        CodeFile { id, name, source }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestFileWrapper {
        content: String,
    }

    impl DynemicFileRead for TestFileWrapper {
        fn get_slice(&self, start: usize, end: usize) -> String {
            self.content
                .clone()
                .chars()
                .skip(start)
                .take(end - start + 1)
                .collect()
        }
        fn get_content(&self) -> String {
            self.content.clone()
        }
    }

    impl DynemicFileWrite for TestFileWrapper {
        fn set_slice(&mut self, start: usize, end: usize, content: String) {
            let mut chars: Vec<char> = self.content.chars().collect();
            chars[start..end].clone_from_slice(&content.chars().collect::<Vec<char>>());
            self.content = chars.into_iter().collect();
        }
        fn set_content(&mut self, content: String) {
            self.content = content;
        }
    }

    impl DynemicFileCreateDelete for TestFileWrapper {
        fn create_file(&self) -> Result<(), std::io::Error> {
            Ok(())
        }
        fn delete_file(&self) -> Result<(), std::io::Error> {
            Ok(())
        }
    }

    #[test]
    fn test_code_file() {
        let file_wrapper = TestFileWrapper {
            content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string(),
        };
        let code_file = CodeFile::new(Uuid::new_v4(), "test_file.txt".to_string(), file_wrapper);
        assert_eq!(
            code_file.source.get_content(),
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string()
        );
        assert_eq!(code_file.source.get_slice(0, 10), "Lorem ipsum".to_string());
    }

    #[test]
    fn test_code_file_set_content() {
        let mut file_wrapper = TestFileWrapper {
            content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string(),
        };
        let mut code_file =
            CodeFile::new(Uuid::new_v4(), "test_file.txt".to_string(), file_wrapper);
        code_file.source.set_content("New content".to_string());
        assert_eq!(code_file.source.get_content(), "New content".to_string());
    }
}
