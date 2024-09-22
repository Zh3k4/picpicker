use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P: AsRef<Path>> FileExtension for P {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }
        false
    }
}

pub fn findfiles(dir: &Path, files: &mut Vec<DirEntry>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;

        match entry.file_name().to_str() {
            Some(f) => {
                if f.starts_with(".") {
                    continue;
                }
            }
            None => continue,
        }

        let path = entry.path();

        if path.is_dir() {
            findfiles(&path, files)?;
        }

        files.push(entry);
    }
    Ok(())
}
