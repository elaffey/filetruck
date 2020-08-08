use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

use super::plan::Plan;

fn append(path: &Path, dir_name: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    path_buf.push(dir_name);
    path_buf
}

fn copy(files: &Vec<String>, input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    for file in files {
        let a = append(input, &file).canonicalize()?;
        if !a.is_file() {
            eprintln!("{:?} is not a file", a);
            let msg = "input is not a file".to_string();
            return Err(msg.into());
        }
        let b = append(output, &file);
        let b_parent = b.parent().ok_or("Could not get parent")?;
        std::fs::create_dir_all(b_parent)?;

        println!("{} -> {}", a.display(), b.display());
        std::fs::copy(a, b)?;
    }
    Ok(())
}

pub fn pick_up(plan: Plan, from: String) -> Result<(), Box<dyn Error>> {
    let cur_dir = env::current_dir()?;
    let output = append(&cur_dir, &plan.name);
    let output = output.as_path();
    std::fs::create_dir_all(&output)?;
    let input = Path::new(&from);

    copy(&plan.files, input, output)?;

    Ok(())
}

pub fn drop_off(plan: Plan, to: String) -> Result<(), Box<dyn Error>> {
    let cur_dir = env::current_dir()?;
    let input = append(&cur_dir, &plan.name);
    let input = input.as_path();
    let output = Path::new(&to);

    copy(&plan.files, input, output)?;

    Ok(())
}
