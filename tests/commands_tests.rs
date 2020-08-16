use std::fs::File;
use std::path::PathBuf;

use filetruck::commands::{drop_off, pick_up};
use filetruck::plan::Plan;

struct TempDir {
    path: PathBuf,
}

impl Drop for TempDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).unwrap();
    }
}

impl TempDir {
    fn new(name: &str) -> TempDir {
        std::fs::create_dir(name).unwrap();
        TempDir {
            path: PathBuf::from(name),
        }
    }

    fn add_file(&self, name: &str) {
        let mut path = self.path.clone();
        path.push(name);
        File::create(path).unwrap();
    }

    fn add_dir(&self, name: &str) {
        let mut path = self.path.clone();
        path.push(name);
        std::fs::create_dir(path).unwrap();
    }
}

#[test]
fn test_pick_up() {
    let plan = Plan {
        name: "plan1".to_string(),
        files: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let dir = TempDir::new("pickup1");
    dir.add_file("a");
    dir.add_file("b");
    dir.add_file("c");

    let res = pick_up(plan, PathBuf::from(&dir.path));
    assert!(res.is_ok());
    std::fs::remove_dir_all("plan1").unwrap();
}

#[test]
fn test_pick_up_same_file() {
    let plan = Plan {
        name: "plan2".to_string(),
        files: vec![],
    };
    let dir = TempDir::new("plan2");

    let res = pick_up(plan, PathBuf::from(&dir.path));
    assert!(res.is_err());
    let msg = res.unwrap_err().to_string();
    assert!(msg.starts_with("Input and output are the same"));
}

#[test]
fn test_pick_up_create_dir_error() {
    let plan = Plan {
        name: "/badpath".to_string(),
        files: vec![],
    };

    let res = pick_up(plan, PathBuf::from("anywhere"));
    assert!(res.is_err());
    let msg = res.unwrap_err().to_string();
    assert!(msg.starts_with("Could not create directories"));
}

#[test]
fn test_pick_up_not_a_file() {
    let plan = Plan {
        name: "plan3".to_string(),
        files: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let dir = TempDir::new("pickup2");
    dir.add_file("a");
    dir.add_file("b");
    dir.add_dir("c");

    let res = pick_up(plan, PathBuf::from(&dir.path));
    assert!(res.is_err());
    let msg = res.unwrap_err().to_string();
    assert!(msg.ends_with("is not a file"));
    std::fs::remove_dir_all("plan3").unwrap();
}

#[test]
fn test_pick_up_error_reading_file() {
    let plan = Plan {
        name: "plan4".to_string(),
        files: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let dir = TempDir::new("pickup3");
    dir.add_file("a");
    dir.add_file("b");

    let res = pick_up(plan, PathBuf::from(&dir.path));
    assert!(res.is_err());
    let msg = res.unwrap_err().to_string();
    assert!(msg.starts_with("Error reading file"));
    std::fs::remove_dir_all("plan4").unwrap();
}

#[test]
fn test_drop_off() {
    let plan = Plan {
        name: "plan6".to_string(),
        files: vec!["a".to_string(), "b".to_string(), "c".to_string()],
    };
    let pickup = TempDir::new("plan6");
    pickup.add_file("a");
    pickup.add_file("b");
    pickup.add_file("c");
    let dropoff = TempDir::new("dropoff1");

    let res = drop_off(plan, PathBuf::from(&dropoff.path));
    assert!(res.is_ok());
}

//tototo
#[test]
fn test_drop_off_same_file_doesnt_exist() {
    let plan = Plan {
        name: "plan7".to_string(),
        files: vec![],
    };

    let res = drop_off(plan, PathBuf::from("plan7"));
    assert!(res.is_err());
    let msg = res.unwrap_err().to_string();
    assert!(msg.starts_with("One of the following paths could not be opened"));
}
