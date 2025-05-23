use crate::spec::MessageDefinition;
use std::collections::HashSet;
use thiserror::Error;

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

pub fn validate_message_numbering(spec: &MessageDefinition) -> Result<(), String> {
    let mut used_numbers = std::collections::HashMap::new();

    for category in &spec.messages {
        for message in &category.messages {
            if let Some(number) = message.number {
                if let Some(existing) = used_numbers.get(&number) {
                    return Err(format!(
                        "Duplicate message number {} used by '{}' and '{}'",
                        number, existing, message.original_name
                    ));
                }
                used_numbers.insert(number, message.original_name.clone());
            }
        }
    }

    Ok(())
}

pub fn validate_schema(yaml_content: &str) -> Result<(), ValidationError> {
    // First, try to parse the YAML as our MessageDefinition type
    let spec: MessageDefinition = serde_yaml::from_str(yaml_content)
        .map_err(|e| ValidationError::ParseError(e.to_string()))?;

    // Now perform additional validation checks

    // Validate settings
    if spec.settings.output_file.is_empty() {
        return Err(ValidationError::MissingField(
            "settings.output_file".to_string(),
        ));
    }

    if spec.settings.module_doc.is_empty() {
        return Err(ValidationError::MissingField(
            "settings.module_doc".to_string(),
        ));
    }

    // Check for valid serialization framework
    let valid_frameworks = ["bincode", "serde_json", "postcard"];
    if !valid_frameworks.contains(&spec.settings.serialization_framework.as_str()) {
        return Err(ValidationError::InvalidValue(format!(
            "settings.serialization_framework '{}' is not one of the supported frameworks: {:?}",
            spec.settings.serialization_framework, valid_frameworks
        )));
    }

    // common_struct section is optional, but if present, validate it
    if let Some(common_structs) = spec.common_structs {
        for (struct_name, struct_def) in common_structs {
            if struct_name.is_empty() {
                return Err(ValidationError::MissingField(
                    "common_structs.name".to_string(),
                ));
            }
            if struct_def.fields.is_empty() {
                return Err(ValidationError::InvalidValue(format!(
                    "Struct '{struct_name}' has no fields"
                )));
            }

            // Check for duplicate field names
            let mut field_names = HashSet::new();
            for field_name in struct_def.fields.keys() {
                if !field_names.insert(field_name) {
                    return Err(ValidationError::InvalidValue(format!(
                        "Struct '{struct_name}' has duplicate field '{field_name}'"
                    )));
                }
            }
        }
    }

    // Enum section is optional, but if present, validate it
    if let Some(enums) = spec.enums {
        for (enum_name, enum_def) in enums {
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
    }

    // error_message section is optional, but if present, validate it
    if let Some(error_messages) = spec.error_message {
        if error_messages.description.is_empty() {
            return Err(ValidationError::MissingField(
                "error_messages.description".to_string(),
            ));
        }
        for (error_name, error_def) in &error_messages.fields {
            if error_name.is_empty() {
                return Err(ValidationError::MissingField(
                    "error_messages.name".to_string(),
                ));
            }
            if error_def.description.is_empty() {
                return Err(ValidationError::MissingField(
                    "error_messages.description".to_string(),
                ));
            }
            if error_def.type_name.is_empty() {
                return Err(ValidationError::MissingField(
                    "error_messages.type_name".to_string(),
                ));
            }
            if error_def.type_name != "String" && error_def.type_name != "u32" {
                return Err(ValidationError::InvalidValue(format!(
                    "Error message '{error_name}' has invalid type '{}'",
                    error_def.type_name
                )));
            }
        }
        // Check for duplicate field names in error_message
        let mut field_names = HashSet::new();
        for field_name in error_messages.fields.keys() {
            if !field_names.insert(field_name) {
                return Err(ValidationError::InvalidValue(format!(
                    "Error message has duplicate field '{field_name}'"
                )));
            }
        }
    }

    // Validate message definitions
    if spec.messages.is_empty() {
        return Err(ValidationError::StructureError(
            "No message categories defined".to_string(),
        ));
    }

    for category in &spec.messages {
        if category.category.is_empty() {
            return Err(ValidationError::MissingField(
                "category.category".to_string(),
            ));
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
                return Err(ValidationError::MissingField(
                    "message.original_name".to_string(),
                ));
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

    // All(most) validation passed
    Ok(())
}
