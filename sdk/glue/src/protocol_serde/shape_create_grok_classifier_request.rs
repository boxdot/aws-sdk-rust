// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_grok_classifier_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateGrokClassifierRequest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.classification {
        object.key("Classification").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.grok_pattern {
        object.key("GrokPattern").string(var_3.as_str());
    }
    if let Some(var_4) = &input.custom_patterns {
        object.key("CustomPatterns").string(var_4.as_str());
    }
    Ok(())
}
