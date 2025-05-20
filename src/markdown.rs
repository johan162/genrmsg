pub use crate::spec::{FieldSpec, MessageDefinition};
pub use std::collections::HashMap;

/// Format payload fields into a readable string for the markdown table
pub fn format_payload_fields(fields: &HashMap<String, FieldSpec>) -> String {
    if fields.is_empty() {
        return "*empty*".to_string();
    }

    // Sort fields by name for consistent ordering
    let mut sorted_fields: Vec<_> = fields.iter().collect();
    sorted_fields.sort_by(|a, b| a.0.cmp(b.0));
    let fields = sorted_fields;

    // Format fields as a list of "name: type" pairs
    let field_strings: Vec<String> = fields
        .iter()
        .map(|(name, field)| format!("`{}`: `{}`", name, field.type_name))
        .collect();

    field_strings.join("<br>")
}

/// Generate a Markdown table of all defined messages
pub fn generate_message_markdown_table(spec: &MessageDefinition) -> String {
    let mut markdown = String::new();

    // Add section title
    markdown.push_str("# Message Specification\n\n");

    // Sequence number for human readability
    let mut seq = 1;

    // Process all message categories
    for category in &spec.messages {
        // Add category as a section header
        markdown.push_str(&format!("\n## {} Messages\n\n", category.category));

        // Add table header again for each category
        markdown.push_str(
            "| # | Request Name | Request Payload | Reply Name | Reply Payload | Description |\n",
        );
        markdown.push_str(
            "|-----|-------------|----------------|------------|---------------|-------------|\n",
        );

        for message in &category.messages {
            let request_name = &message.generated_name;
            let response_name = format!("{request_name}_Reply");

            // Format payload fields
            let request_payload = format_payload_fields(&message.request.fields);
            let reply_payload = format_payload_fields(&message.response.fields);

            // Add row
            markdown.push_str(&format!(
                "| {} | `{}` | {} | `{}` | {} | {} |\n",
                seq,
                request_name,
                request_payload,
                response_name,
                reply_payload,
                message.description
            ));

            seq += 1;
        }
    }

    markdown
}
