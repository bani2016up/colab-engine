

pub struct ViewportRequest{
    pub start_index: u64,
    pub end_index: u64,
    pub content: String
}

pub struct UpdateCodeRequest{
    pub start: u64,
    pub end: u64,
    pub content: String
}

pub struct CreateCodeFileRequest{
    pub name: String
}

pub struct CodeFileResponse{
    pub name: String,
    pub viewport: ViewportRequest
}
