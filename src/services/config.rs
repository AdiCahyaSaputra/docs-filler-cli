use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Placeholder {
    pub prefix: String,
    pub suffix: String,
}

#[derive(Deserialize, Debug)]
pub struct ExcelConfig {
    pub sheet_name: String,
    pub placeholder: Placeholder,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub excel: ExcelConfig,
}

impl Config {
    pub fn get() -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string("config.json")?;
        let data: Self = serde_json::from_str(&json)?;

        return Ok(data);
    }
}
