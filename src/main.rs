use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

mod files;
use files::FileExtension;

const SOURCE_PATH: &str = "";
const DESTINATION_PATH: &str = "";

fn main() -> io::Result<()> {
    let mut files = Vec::new();
    files::findfiles(Path::new(SOURCE_PATH), &mut files)?;

    let mut files = {
        let mut vec = Vec::new();
        for f in files {
            if f.path().has_extension(&["png", "jpg", "jpeg", "bmp"]) {
                vec.push(f);
            }
        }
        vec
    };

    files.sort_by(|a, b| {
        let actime = a.metadata().unwrap().created().unwrap();
        let bctime = b.metadata().unwrap().created().unwrap();

        Ord::cmp(&actime, &bctime).reverse()
    });

    let files = {
        let mut vec = Vec::new();
        for f in files {
            if let Ok(s) = f.path().into_os_string().into_string() {
                vec.push(s);
            }
        }
        vec
    };

    if let Some(select) = sxiv(&files)? {
        for s in select {
            let src = Path::new(&s);
            if src.file_name().is_none() {
                continue;
            }
            let mut dst = PathBuf::new();
            dst.push(DESTINATION_PATH);
            dst.push(src.file_name().unwrap());
            fs::copy(src, dst)?;
        }
    }

    Ok(())
}

fn sxiv(files: &[String]) -> io::Result<Option<Vec<String>>> {
    let mut sxiv = Command::new("sxiv")
        .arg("-top")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    match sxiv.stdin.as_mut() {
        Some(stdin) => stdin.write_all(files.join("\n").as_bytes())?,
        None => return Ok(None),
    }

    sxiv.wait()?;

    let mut output = String::new();
    match sxiv.stdout {
        Some(mut stdout) => stdout.read_to_string(&mut output)?,
        None => return Ok(None),
    };
    let files = output.split("\n").map(str::to_string).collect();
    Ok(Some(files))
}
