pub struct GetFileResponse{
    pub(crate) filename: String,
     pub(crate) filepath: String
}
pub struct FileSystem {
    files: Vec<String>,
    folders: Vec<FileSystem>
}