use std::env;
use std::path::{Path, PathBuf};

use super::plan::Plan;
use super::error::Error;

fn append(path: &Path, dir_name: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    path_buf.push(dir_name);
    path_buf
}

fn canonicalize(path: &Path) -> Result<PathBuf, Error> {
    path.canonicalize()
        .map_err(|e| Error::new(format!("Error reading file {} - {}", path.display(), e)))
}

fn is_file(path: &Path) -> Result<(), Error> {
    let path = canonicalize(path)?;
    if !path.is_file() {
        return Err(Error::new(format!("{} is not a file", path.display())));
    }
    Ok(())
}

fn get_parent<'a>(path: &'a Path) -> Result<&'a Path, Error> {
    path.parent().ok_or(Error::new(format!(
        "Could not get parent of {}",
        path.display()
    )))
}

fn copy(a: &Path, b: &Path) -> Result<(), Error> {
    std::fs::copy(a, b).map_err(|e| {
        Error::new(format!(
            "Could not copy {} -> {} - {}",
            a.display(),
            b.display(),
            e
        ))
    })?;
    Ok(())
}

fn create_dir_all(path: &Path) -> Result<(), Error> {
    std::fs::create_dir_all(path)
        .map_err(|e| Error::new(format!("Could not create directories {}", e)))?;
    Ok(())
}

fn create_parent_dirs(path: &Path) -> Result<(), Error> {
    let parent = get_parent(path)?;
    create_dir_all(parent)?;
    Ok(())
}

fn copy_file(file: &str, input: &Path, output: &Path) -> Result<(), Error> {
    let a = append(input, file);
    is_file(&a)?;
    let b = append(output, &file);
    create_parent_dirs(&b)?;
    copy(&a, &b)?;
    println!("{} -> {}", a.display(), b.display());
    Ok(())
}

fn copy_files(files: &Vec<String>, input: &Path, output: &Path) {
    for file in files {
        match copy_file(file, input, output) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

fn current_dir() -> Result<PathBuf, Error> {
    env::current_dir().map_err(|e| Error::new(format!("Could not get current dir {}", e)))
}

pub fn pick_up(plan: Plan, from: String) -> Result<(), Error> {
    let cur_dir = current_dir()?;
    let output = append(&cur_dir, &plan.name);
    let output = output.as_path();
    create_dir_all(&output)?;
    let input = Path::new(&from);

    copy_files(&plan.files, input, output);

    Ok(())
}

pub fn drop_off(plan: Plan, to: String) -> Result<(), Error> {
    let cur_dir = current_dir()?;
    let input = append(&cur_dir, &plan.name);
    let input = input.as_path();
    let output = Path::new(&to);

    copy_files(&plan.files, input, output);

    Ok(())
}
