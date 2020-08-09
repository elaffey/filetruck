use serde::Deserialize;

use super::error::Error;

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub name: String,
    pub files: Vec<String>,
}

impl Plan {
    pub fn load(path: &str) -> Result<Self, Error> {
        let yaml_str = std::fs::read_to_string(path)
            .map_err(|e| Error::new(format!("Error reading plan file {} - {}", path, e)))?;
        let ret: Self = serde_yaml::from_str(&yaml_str)
            .map_err(|e| Error::new(format!("Error parsing plan {}", e)))?;
        Ok(ret)
    }
}
