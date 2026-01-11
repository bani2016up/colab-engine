use std::{collections::HashMap};

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::domain::traits::merge::Mergable;


const DEFUALT_CONTENT_MAX_LENGTH: usize = 1000;

#[derive(Debug, Clone)]
pub struct CodeFile {
    pub name: String,
    pub content: HashMap<u64, FilePartialContent>,
}

#[derive(Debug, Clone)]
pub struct FilePartialContent {
    hash: u64,
    content: String
}


impl CodeFile {
    fn new(name: String, content: String) -> Self {
        CodeFile::new_with_chunk_len(name, content, DEFUALT_CONTENT_MAX_LENGTH)
    }

     fn new_with_chunk_len(name: String, content: String, chunk_len: usize) -> Self {
        let mut content_parts: HashMap<u64, FilePartialContent> = HashMap::new();
        for i in 0..(content.len() / chunk_len + 1) {
            let start = chunk_len * i;
            let end = std::cmp::min(start + chunk_len, content.len());
            if start < content.len() {
                let file_partial_content = FilePartialContent::new(content[start..end].to_string());
                content_parts.insert(file_partial_content.get_hash(), file_partial_content);
            }
        }
        CodeFile {
            name,
            content: content_parts,
        }
    }
}



impl FilePartialContent {
    pub fn new(content: String) -> Self {
        FilePartialContent {
            hash: FilePartialContent::evaluate_hash(&content),
            content,
        }
    }

     pub fn evaluate_hash(content: &str) -> u64 {
        let mut state = DefaultHasher::new();
        content.as_bytes().hash(&mut state);
        state.finish()
    }

    pub fn get_hash(&self) -> u64 {
        self.hash
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content.clone();
        self.hash = FilePartialContent::evaluate_hash(&content);
    }
}


impl Mergable for FilePartialContent {
    fn merge(&self, other: &Self) -> Self {
        let content1 = self.get_content();
        let content2 = other.get_content();
        let merged_content = format!("{}{}", content1, content2);
        FilePartialContent::new(merged_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_file_creation() {
        let code_file = CodeFile::new("test_file".to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string());
        assert_eq!(code_file.name, "test_file");
        assert_eq!(code_file.content.len(), 1);
        let content_part = code_file.content.values().next().unwrap();
        assert_eq!(content_part.get_content(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    }

    #[test]
    fn test_code_file_creation_with_chunk_len() {
        let code_file = CodeFile::new_with_chunk_len("test_file".to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string(), 10);
        assert_eq!(code_file.name, "test_file");
        assert_eq!(code_file.content.len(), 6);
        let mut contents: Vec<&str> = code_file.content.values().map(|v| v.get_content()).collect();
        contents.sort();
        assert!(contents.contains(&"Lorem ipsu"));
    }

    #[test]
    fn test_file_partial_content_hash() {
        let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();
        let file_partial_content = FilePartialContent::new(content.clone());
        assert_eq!(file_partial_content.get_hash(), FilePartialContent::evaluate_hash(&content));
    }

    #[test]
    fn test_file_partial_content_hash_func() {
        let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();
        let mut hasher = DefaultHasher::new();
        content.as_bytes().hash(&mut hasher);
        assert_eq!(FilePartialContent::evaluate_hash(&content), hasher.finish());
    }

    #[test]
    fn test_file_partial_content_merge() {
        let content1 = "Lorem ipsum dolor sit amet, ".to_string();
        let content2 = "consectetur adipiscing elit.".to_string();
        let file_partial_content1 = FilePartialContent::new(content1.clone());
        let file_partial_content2 = FilePartialContent::new(content2.clone());
        let merged_content = file_partial_content1.merge(&file_partial_content2);
        assert_eq!(merged_content.get_content(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    }
}
