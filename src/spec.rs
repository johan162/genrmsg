pub use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub output_file: String,
    pub module_doc: String,
    pub serialization_framework: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSpec {
    #[serde(rename = "type")]
    pub type_name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructSpec {
    pub description: String,
    pub fields: HashMap<String, FieldSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumSpec {
    pub description: String,
    pub variants: Vec<String>,
    #[serde(default)]
    pub implement_traits: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageFieldsSpec {
    pub fields: HashMap<String, FieldSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSpec {
    #[serde(rename = "name")]
    pub original_name: String,
    #[serde(skip, default)]
    pub generated_name: String, // Will store the auto-numbered name
    pub description: String,
    pub request: MessageFieldsSpec,
    pub response: MessageFieldsSpec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageCategory {
    pub category: String,
    pub messages: Vec<MessageSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDefinition {
    pub settings: Settings,
    pub common_structs: HashMap<String, StructSpec>,
    pub enums: HashMap<String, EnumSpec>,
    pub error_message: StructSpec,
    pub messages: Vec<MessageCategory>,
}
