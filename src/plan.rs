use serde::Deserialize;
use std::path::Path;

use super::error::Error;

#[derive(Debug, Deserialize)]
pub struct Plan {
    pub name: String,
    pub files: Vec<String>,
}

impl Plan {
    pub fn load(path: &Path) -> Result<Plan, Error> {
        let yaml_str = std::fs::read_to_string(path).map_err(|e| {
            Error::new(format!(
                "Error reading plan file {} - {}",
                path.display(),
                e
            ))
        })?;
        let plan: Plan = serde_yaml::from_str(&yaml_str)
            .map_err(|e| Error::new(format!("Error parsing plan - {}", e)))?;

        for file in &plan.files {
            if file.contains("../") || file.contains("/..") {
                return Err(Error::new(format!("Files cannot contain .. - {}", file)));
            }
        }

        Ok(plan)
    }
}
