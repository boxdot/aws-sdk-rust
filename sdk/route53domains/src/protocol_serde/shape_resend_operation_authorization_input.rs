// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resend_operation_authorization_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::resend_operation_authorization::ResendOperationAuthorizationInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.operation_id {
        object.key("OperationId").string(var_1.as_str());
    }
    Ok(())
}
