use uuid::Uuid;

use std::{collections::HashMap};

use std::hash::{DefaultHasher, Hash, Hasher};


#[derive(Debug, Clone)]
pub struct CodeFile {
    id: Uuid,
    pub name: String,
    pub content: HashMap<u64, FilePartialContent>,
}

#[derive(Debug, Clone)]
pub struct FilePartialContent {
    id: Uuid,
    hash: u64,
    content: String
}


impl CodeFile {
     fn new(id: Uuid, name: String, content: HashMap<u64, FilePartialContent>) -> Self {
        CodeFile {
            id: id,
            name: name,
            content: content,
        }
    }
}




impl FilePartialContent {
    pub fn new(id: Uuid, content: String) -> Self {
        FilePartialContent {
            id: id,
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_file_creation() {
        let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();
        let file_partial_content = FilePartialContent::new(Uuid::new_v4(), content.clone());
        let hash = file_partial_content.get_hash();

        let mut content_map = HashMap::new();
        content_map.insert(hash, file_partial_content);

        let code_file = CodeFile::new(Uuid::new_v4(), "test_file".to_string(), content_map);
        assert_eq!(code_file.name, "test_file");
        assert_eq!(code_file.content.len(), 1);
        let content_part = code_file.content.values().next().unwrap();
        assert_eq!(content_part.get_content(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    }

    #[test]
    fn test_code_file_creation_with_multiple_chunks() {
        let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
        let chunk_len = 10;
        let mut content_map = HashMap::new();

        for i in 0..(content.len() / chunk_len + 1) {
            let start = chunk_len * i;
            let end = std::cmp::min(start + chunk_len, content.len());
            if start < content.len() {
                let chunk = &content[start..end];
                let file_partial_content = FilePartialContent::new(Uuid::new_v4(), chunk.to_string());
                content_map.insert(file_partial_content.get_hash(), file_partial_content);
            }
        }

        let code_file = CodeFile::new(Uuid::new_v4(), "test_file".to_string(), content_map);
        assert_eq!(code_file.name, "test_file");
        assert_eq!(code_file.content.len(), 6);
        let mut contents: Vec<&str> = code_file.content.values().map(|v| v.get_content()).collect();
        contents.sort();
        assert!(contents.contains(&"Lorem ipsu"));
    }

    #[test]
    fn test_file_partial_content_hash() {
        let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();
        let file_partial_content = FilePartialContent::new(Uuid::new_v4(), content.clone());
        assert_eq!(file_partial_content.get_hash(), FilePartialContent::evaluate_hash(&content));
    }

    #[test]
    fn test_file_partial_content_hash_func() {
        let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".to_string();
        let mut hasher = DefaultHasher::new();
        content.as_bytes().hash(&mut hasher);
        assert_eq!(FilePartialContent::evaluate_hash(&content), hasher.finish());
    }
}
