use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub name: String,
    pub files: Vec<String>,
}

impl Plan {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let yaml_str = std::fs::read_to_string(path)?;
        let ret: Self = serde_yaml::from_str(&yaml_str)?;
        Ok(ret)
    }
}
