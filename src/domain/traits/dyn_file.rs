pub trait DynemicFileCreateDelete {
    fn create_file(&self) -> Result<(), std::io::Error>;
    fn delete_file(&self) -> Result<(), std::io::Error>;
}

pub trait DynemicFileRead {
    fn get_slice(&self, start: usize, end: usize) -> String;
    fn get_content(&self) -> String;
}

pub trait DynemicFileWrite {
    fn set_slice(&mut self, start: usize, end: usize, content: String);
    fn set_content(&mut self, content: String);
}
