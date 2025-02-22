// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_log_pattern_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_log_pattern::DescribeLogPatternInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pattern_set_name {
        object.key("PatternSetName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.pattern_name {
        object.key("PatternName").string(var_3.as_str());
    }
    Ok(())
}
