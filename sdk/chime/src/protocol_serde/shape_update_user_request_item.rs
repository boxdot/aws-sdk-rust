// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_user_request_item(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateUserRequestItem,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.user_id {
        object.key("UserId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.license_type {
        object.key("LicenseType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.user_type {
        object.key("UserType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.alexa_for_business_metadata {
        #[allow(unused_mut)]
        let mut object_5 = object.key("AlexaForBusinessMetadata").start_object();
        crate::protocol_serde::shape_alexa_for_business_metadata::ser_alexa_for_business_metadata(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    Ok(())
}
