
#[derive(Debug)]
pub enum ApplicationError {
    FileNotFound(String),
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    InvalidRange(usize),
}
