use std::{fs, vec};

use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct PlaceholderData {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String, // "common" or "computed". I dont use enum in there cuz i can use string into pattern matching directly
    pub value: String,
}

#[derive(Debug)]
pub struct ParsedPlaceholder {
    pub name: String,
    pub value: String,
}

pub type PlaceholderDataJSON = Vec<Vec<PlaceholderData>>;
pub type ParsedResult = Vec<Vec<ParsedPlaceholder>>;

impl PlaceholderData {
    pub fn get_json_value(path: &Path) -> Result<PlaceholderDataJSON, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let data: PlaceholderDataJSON = serde_json::from_str(&json)?;

        return Ok(data);
    }
}

pub fn parsing_template(path: &Path) -> ParsedResult {
    let placeholder_data = PlaceholderData::get_json_value(path).unwrap();
    let mut parsed_result: ParsedResult = vec![];

    for placeholders in placeholder_data {
        let mut parsed_placeholder: Vec<ParsedPlaceholder> = vec![];

        for placeholder in placeholders {
            match placeholder.data_type.as_str() {
                "common" => {
                    parsed_placeholder.push(ParsedPlaceholder {
                        name: placeholder.name.clone(),
                        value: placeholder.value.clone(),
                    });
                }
                // TODO: IMPLEMENT COMPUTED VALUE LATER... MUST WRITE PARSER....
                // "computed" => {
                // }
                _ => {}
            };
        }

        parsed_result.push(parsed_placeholder);
    }

    return parsed_result;
}
