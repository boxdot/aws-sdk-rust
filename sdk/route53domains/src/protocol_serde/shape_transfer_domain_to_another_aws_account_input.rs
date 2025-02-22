// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transfer_domain_to_another_aws_account_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::transfer_domain_to_another_aws_account::TransferDomainToAnotherAwsAccountInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("DomainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.account_id {
        object.key("AccountId").string(var_2.as_str());
    }
    Ok(())
}
