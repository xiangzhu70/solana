use {
    //log::info,
    std::path::{Path, PathBuf},
    thiserror::Error,
};

//use std::fs::File;

#[derive(Error, Debug)]
pub enum FsWrapError {
    #[error("IO error in function {function} at {file}:{line} for path: {path:?} - {source}")]
    IoError {
        source: std::io::Error,
        path: PathBuf,
        function: &'static str,
        file: &'static str,
        line: u32,
    },
}

impl From<std::io::Error> for FsWrapError {
    fn from(error: std::io::Error) -> Self {
        FsWrapError::IoError {
            source: error,
            path: PathBuf::new(),
            function: file!(),
            file: file!(),
            line: line!(),
        }
    }
}

fn wrap_file_open<P: AsRef<Path>>(
    path: P,
    function: &'static str,
    file: &'static str,
    line: u32,
) -> Result<std::fs::File, FsWrapError> {
    let path = path.as_ref();
    std::fs::File::open(path).map_err(|source| FsWrapError::IoError {
        source,
        path: path.to_path_buf(),
        function,
        file,
        line,
    })
}

pub struct File {}

impl File {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<std::fs::File, FsWrapError> {
        let file = wrap_file_open(path, "open", file!(), line!())?;
        Ok(file)
    }
}
