use std::env;
use std::path::{Path, PathBuf};

use super::plan::Plan;

fn append(path: &Path, dir_name: &str) -> PathBuf {
    let mut path_buf = PathBuf::from(path);
    path_buf.push(dir_name);
    path_buf
}

fn copy(files: &Vec<String>, input: &Path, output: &Path) -> std::io::Result<()> {
    for file in files {
        let a = append(input, &file);
        match a.canonicalize() {
            Ok(a) => {
                if !a.is_file() {
                    eprintln!("ERROR: {} is not a file", a.display());
                    break;
                }
                let b = append(output, &file);
                match b.parent() {
                    Some(b_parent) => {
                        std::fs::create_dir_all(b_parent)?;

                        println!("{} -> {}", a.display(), b.display());
                        std::fs::copy(a, b)?;
                    }
                    None => {
                        eprintln!("ERROR: Could not get parent of {}", b.display());
                    }
                }
            }
            Err(e) => {
                eprintln!("ERROR: Error reading file {} - {}", a.display(), e);
            }
        }
    }
    Ok(())
}

pub fn pick_up(plan: Plan, from: String) -> std::io::Result<()> {
    let cur_dir = env::current_dir()?;
    let output = append(&cur_dir, &plan.name);
    let output = output.as_path();
    std::fs::create_dir_all(&output)?;
    let input = Path::new(&from);

    copy(&plan.files, input, output)?;

    Ok(())
}

pub fn drop_off(plan: Plan, to: String) -> std::io::Result<()> {
    let cur_dir = env::current_dir()?;
    let input = append(&cur_dir, &plan.name);
    let input = input.as_path();
    let output = Path::new(&to);

    copy(&plan.files, input, output)?;

    Ok(())
}
