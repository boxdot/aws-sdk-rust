// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_parameter_group_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_parameter_group::DeleteParameterGroupInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_1.as_str());
    }
    Ok(())
}
