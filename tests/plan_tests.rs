mod util;

use std::path::Path;

use filetruck::plan::Plan;
use util::TempFile;

#[test]
fn test_load_plan_error_reading_file() {
    let name = "idontexist.yml";
    let path = Path::new(name);
    let res = Plan::load(path);
    assert!(res.is_err());
    let msg = res.unwrap_err().to_string();
    assert!(msg.starts_with("Error reading plan file"));
}

#[test]
fn test_load_plan_error_parsing() {
    let plan = "
    name: test
    file:
      - a.txt
      - b.txt
      - subdir/c.txt
    ";
    let name = "a.yml";
    let temp = TempFile::new(name, plan).unwrap();
    let res = Plan::load(temp.path());
    assert!(res.is_err());
    let msg = res.unwrap_err().to_string();
    assert!(msg.starts_with("Error parsing plan"));
}

#[test]
fn test_load_plan() {
    let plan = "
    name: test
    files:
      - a.txt
      - b.txt
      - subdir/c.txt
    ";
    let name = "b.yml";
    let temp = TempFile::new(name, plan).unwrap();
    let res = Plan::load(temp.path());
    assert!(res.is_ok());
    let plan = res.unwrap();
    assert_eq!(plan.name, "test");
    assert_eq!(plan.files.len(), 3);
    assert_eq!(plan.files[0], "a.txt");
    assert_eq!(plan.files[1], "b.txt");
    assert_eq!(plan.files[2], "subdir/c.txt");
}
