use crate::domain::traits::dyn_file::{DynemicFileCreateDelete, DynemicFileRead, DynemicFileWrite};
use memmap2::{Mmap, MmapMut};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub struct MmapFileSystemSource {
    pub path: PathBuf,
    pub mmap: Option<Mmap>,
}

impl MmapFileSystemSource {
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let file = File::open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self {
            path,
            mmap: Some(mmap),
        })
    }
}

impl Clone for MmapFileSystemSource {
    fn clone(&self) -> Self {
        if self.mmap.is_some() {
            Self::new(self.path.clone()).expect("Failed to clone MmapFileSystemSource")
        } else {
            Self {
                path: self.path.clone(),
                mmap: None,
            }
        }
    }
}

impl DynemicFileRead for MmapFileSystemSource {
    fn get_slice(&self, start: usize, end: usize) -> String {
        if let Some(mmap) = &self.mmap {
            let slice = &mmap[start..end];
            String::from_utf8_lossy(slice).to_string()
        } else {
            String::new()
        }
    }

    fn get_content(&self) -> String {
        if let Some(mmap) = &self.mmap {
            String::from_utf8_lossy(&mmap[..]).to_string()
        } else {
            String::new()
        }
    }
}

impl DynemicFileWrite for MmapFileSystemSource {
    fn set_slice(&mut self, start: usize, end: usize, content: String) {
        let current_content = self.get_content();
        let mut chars: Vec<char> = current_content.chars().collect();
        let new_chars: Vec<char> = content.chars().collect();

        chars.splice(start..end, new_chars.iter().cloned());

        let new_content: String = chars.into_iter().collect();
        self.set_content(new_content);
    }

    fn set_content(&mut self, content: String) {
        std::fs::write(&self.path, content).expect("Failed to write file");
        let file = File::open(&self.path).expect("Failed to open file");
        self.mmap = Some(unsafe { Mmap::map(&file).expect("Failed to map file") });
    }
}

impl DynemicFileCreateDelete for MmapFileSystemSource {
    fn create_file(&self) -> Result<(), std::io::Error> {
        File::create(&self.path).map(|_| ())
    }

    fn delete_file(&self) -> Result<(), std::io::Error> {
        std::fs::remove_file(&self.path).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let file_path = dir.path().join(name);
        let mut file = fs::File::create(&file_path).expect("Failed to create test file");
        file.write_all(content.as_bytes())
            .expect("Failed to write test content");
        file_path
    }

    #[test]
    fn test_new_mmap_file_system_source() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "test.txt", "Hello, World!");

        let source = MmapFileSystemSource::new(file_path);
        assert!(source.is_ok());
    }

    #[test]
    fn test_new_with_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/file.txt");
        let source = MmapFileSystemSource::new(path);
        assert!(source.is_err());
    }

    #[test]
    fn test_get_content() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
        let file_path = create_test_file(&temp_dir, "test.txt", content);

        let source = MmapFileSystemSource::new(file_path).expect("Failed to create source");
        assert_eq!(source.get_content(), content);
    }

    #[test]
    fn test_get_content_empty_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "empty.txt", "");

        let source = MmapFileSystemSource::new(file_path).expect("Failed to create source");
        assert_eq!(source.get_content(), "");
    }

    #[test]
    fn test_get_slice() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let content = "Lorem ipsum dolor sit amet";
        let file_path = create_test_file(&temp_dir, "test.txt", content);

        let source = MmapFileSystemSource::new(file_path).expect("Failed to create source");
        assert_eq!(source.get_slice(0, 5), "Lorem");
        assert_eq!(source.get_slice(6, 11), "ipsum");
        assert_eq!(source.get_slice(0, 11), "Lorem ipsum");
    }

    #[test]
    fn test_get_slice_full_content() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let content = "Hello, World!";
        let file_path = create_test_file(&temp_dir, "test.txt", content);

        let source = MmapFileSystemSource::new(file_path).expect("Failed to create source");
        assert_eq!(source.get_slice(0, content.len()), content);
    }

    #[test]
    fn test_set_content() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "test.txt", "Initial content");

        let mut source =
            MmapFileSystemSource::new(file_path.clone()).expect("Failed to create source");
        let new_content = "New content";
        source.set_content(new_content.to_string());

        assert_eq!(source.get_content(), new_content);

        let file_content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(file_content, new_content);
    }

    #[test]
    fn test_set_content_empty() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "test.txt", "Some content");

        let mut source =
            MmapFileSystemSource::new(file_path.clone()).expect("Failed to create source");
        source.set_content("".to_string());

        assert_eq!(source.get_content(), "");

        let file_content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(file_content, "");
    }

    #[test]
    fn test_set_slice() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "test.txt", "Hello, World!");

        let mut source =
            MmapFileSystemSource::new(file_path.clone()).expect("Failed to create source");
        source.set_slice(0, 6, "Goodbye".to_string());

        let file_content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(file_content, "Goodbye World!");
    }

    #[test]
    fn test_set_slice_middle() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "test.txt", "Lorem ipsum dolor");

        let mut source =
            MmapFileSystemSource::new(file_path.clone()).expect("Failed to create source");
        source.set_slice(6, 11, "XXXXX".to_string());

        let file_content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(file_content, "Lorem XXXXX dolor");
    }

    #[test]
    fn test_multiple_operations() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "test.txt", "Initial content");

        let mut source =
            MmapFileSystemSource::new(file_path.clone()).expect("Failed to create source");

        assert_eq!(source.get_content(), "Initial content");

        source.set_content("Modified content".to_string());
        assert_eq!(source.get_content(), "Modified content");

        assert_eq!(source.get_slice(0, 8), "Modified");

        source.set_slice(0, 8, "Changed!".to_string());
        let file_content = fs::read_to_string(&file_path).expect("Failed to read file");
        assert_eq!(file_content, "Changed! content");
    }

    #[test]
    fn test_unicode_content() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let content = "Hello 世界! 🦀";
        let file_path = create_test_file(&temp_dir, "test.txt", content);

        let source = MmapFileSystemSource::new(file_path).expect("Failed to create source");
        assert_eq!(source.get_content(), content);
    }

    #[test]
    fn test_large_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let content = "A".repeat(10000);
        let file_path = create_test_file(&temp_dir, "large.txt", &content);

        let source = MmapFileSystemSource::new(file_path).expect("Failed to create source");
        assert_eq!(source.get_content().len(), 10000);
        assert_eq!(source.get_slice(0, 100), "A".repeat(100));
    }

    #[test]
    fn test_create_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("new_file.txt");

        let source = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        let result = source.create_file();
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn test_create_file_already_exists() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "existing.txt", "content");

        let source = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        let result = source.create_file();
        assert!(result.is_ok());
        assert!(file_path.exists());
    }

    #[test]
    fn test_create_file_invalid_path() {
        let file_path = PathBuf::from("/invalid/nonexistent/directory/file.txt");

        let source = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        let result = source.create_file();
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "to_delete.txt", "content");

        assert!(file_path.exists());

        let source = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        let result = source.delete_file();
        assert!(result.is_ok());
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_nonexistent_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("nonexistent.txt");

        let source = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        let result = source.delete_file();
        assert!(result.is_err());
    }

    #[test]
    fn test_create_then_delete_file() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = temp_dir.path().join("temp_file.txt");

        let source = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        let create_result = source.create_file();
        assert!(create_result.is_ok());
        assert!(file_path.exists());

        let delete_result = source.delete_file();
        assert!(delete_result.is_ok());
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_file_with_mmap() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = create_test_file(&temp_dir, "mapped.txt", "content");

        let source = MmapFileSystemSource::new(file_path.clone()).expect("Failed to create source");

        drop(source);

        let source_for_delete = MmapFileSystemSource {
            path: file_path.clone(),
            mmap: None,
        };

        let result = source_for_delete.delete_file();
        assert!(result.is_ok());
        assert!(!file_path.exists());
    }
}
