use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct TempFile {
    path: PathBuf,
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let res = std::fs::remove_file(&self.path);
        match res {
            Ok(_) => {}
            Err(e) => eprintln!("Error dropping file {} - {}", self.path.display(), e),
        }
    }
}

impl TempFile {
    pub fn new(name: &str, body: &str) -> std::io::Result<TempFile> {
        let mut file = File::create(name)?;
        file.write_all(body.as_bytes())?;
        let temp_file = TempFile {
            path: PathBuf::from(name),
        };
        Ok(temp_file)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
