
use std::collections::HashSet;
use thiserror::Error;
use crate::spec::MessageDefinition;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("YAML parsing error: {0}")]
    ParseError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid field value: {0}")]
    InvalidValue(String),

    #[error("Schema structure error: {0}")]
    StructureError(String),
}

pub fn validate_schema(yaml_content: &str) -> Result<(), ValidationError> {
    // First, try to parse the YAML as our MessageDefinition type
    let spec: MessageDefinition = serde_yaml::from_str(yaml_content)
        .map_err(|e| ValidationError::ParseError(e.to_string()))?;

    // Now perform additional validation checks
    
    // Validate settings
    if spec.settings.output_file.is_empty() {
        return Err(ValidationError::MissingField("settings.output_file".to_string()));
    }
    
    if spec.settings.module_doc.is_empty() {
        return Err(ValidationError::MissingField("settings.module_doc".to_string()));
    }
    
    // Check for valid serialization framework
    let valid_frameworks = ["bincode", "serde_json", "postcard"];
    if !valid_frameworks.contains(&spec.settings.serialization_framework.as_str()) {
        return Err(ValidationError::InvalidValue(format!(
            "settings.serialization_framework '{}' is not one of the supported frameworks: {:?}",
            spec.settings.serialization_framework, valid_frameworks
        )));
    }

    // Validate enum definitions
    for (enum_name, enum_def) in &spec.enums {
        if enum_name.is_empty() {
            return Err(ValidationError::MissingField("enum.name".to_string()));
        }
        if enum_def.variants.is_empty() {
            return Err(ValidationError::InvalidValue(format!(
                "Enum '{enum_name}' has no variants"
            )));
        }

        // Check for duplicate variants
        let mut variants = HashSet::new();
        for variant in &enum_def.variants {
            if !variants.insert(variant) {
                return Err(ValidationError::InvalidValue(format!(
                    "Enum '{enum_name}' has duplicate variant '{variant}'"
                )));
            }
        }
    }

    // Validate message definitions
    if spec.messages.is_empty() {
        return Err(ValidationError::StructureError("No message categories defined".to_string()));
    }

    for category in &spec.messages {
        if category.category.is_empty() {
            return Err(ValidationError::MissingField("category.category".to_string()));
        }
        
        if category.messages.is_empty() {
            return Err(ValidationError::StructureError(format!(
                "Category '{}' has no messages",
                category.category
            )));
        }

        // Check for message name uniqueness across all categories
        let mut message_names = HashSet::new();
        for message in &category.messages {
            if message.original_name.is_empty() {
                return Err(ValidationError::MissingField("message.original_name".to_string()));
            }
            
            if !message_names.insert(&message.original_name) {
                return Err(ValidationError::InvalidValue(format!(
                    "Duplicate message name: '{}'",
                    message.original_name
                )));
            }

            // ToDo: Check request and response fields
            
        }
    }

    // All validation passed
    Ok(())
}
