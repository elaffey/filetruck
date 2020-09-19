use std::env;
use std::path::{Path, PathBuf};

use super::error::Error;
use super::plan::Plan;
use super::printer::Print;

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

fn get_parent(path: &Path) -> Result<&Path, Error> {
    path.parent()
        .ok_or_else(|| Error::new(format!("Could not get parent of {}", path.display())))
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

fn current_dir() -> Result<PathBuf, Error> {
    env::current_dir().map_err(|e| Error::new(format!("Could not get current dir {}", e)))
}

fn check_not_same_file(a: &Path, b: &Path) -> Result<(), Error> {
    let is_same_file = same_file::is_same_file(a, b).map_err(|e| {
        Error::new(format!(
            "One of the following paths could not be opened {}, {} - {}",
            a.display(),
            b.display(),
            e
        ))
    })?;
    if is_same_file {
        return Err(Error::new(format!(
            "Input and output are the same - {}",
            a.display()
        )));
    }
    Ok(())
}

fn copy_file_name(
    file: &str,
    mut from: PathBuf,
    mut to: PathBuf,
    copied: &mut Vec<(String, String)>,
) -> Result<(), Error> {
    from.push(file);
    to.push(file);
    is_file(&from)?;
    create_parent_dirs(&to)?;
    copy(&from, &to)?;
    let from_str = from.display().to_string();
    let to_str = to.display().to_string();
    copied.push((from_str, to_str));
    Ok(())
}

fn print_copied(copied: Vec<(String, String)>, stdout: &mut impl Print) {
    let mut longest_from = 0;
    for (from, _) in &copied {
        longest_from = std::cmp::max(longest_from, from.len());
    }
    for (from, to) in copied {
        let num_of_spaces = longest_from - from.len() + 1;
        let spaces: String = std::iter::repeat(" ").take(num_of_spaces).collect();
        let to_print = format!("{}{}-> {}", from, spaces, to);
        stdout.writeln(to_print);
    }
    stdout.print();
}

fn copy_file_names(
    files: &[String],
    from: &PathBuf,
    to: &PathBuf,
    stdout: &mut impl Print,
) -> Result<(), Error> {
    let mut copied: Vec<(String, String)> = Vec::new();
    for file in files {
        copy_file_name(file, from.clone(), to.clone(), &mut copied)?;
    }
    print_copied(copied, stdout);
    Ok(())
}

pub fn pick_up(plan: &Plan, from: &PathBuf, stdout: &mut impl Print) -> Result<(), Error> {
    let mut to = current_dir()?;
    to.push(&plan.name);
    create_dir_all(&to)?;
    check_not_same_file(&from, &to)?;
    copy_file_names(&plan.files, &from, &to, stdout)?;
    Ok(())
}

pub fn drop_off(plan: &Plan, to: &PathBuf, stdout: &mut impl Print) -> Result<(), Error> {
    let mut from = current_dir()?;
    from.push(&plan.name);
    check_not_same_file(&from, &to)?;
    copy_file_names(&plan.files, &from, &to, stdout)?;
    Ok(())
}
