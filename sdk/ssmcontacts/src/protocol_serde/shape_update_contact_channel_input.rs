// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_contact_channel_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_contact_channel::UpdateContactChannelInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.contact_channel_id {
        object.key("ContactChannelId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.delivery_address {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DeliveryAddress").start_object();
        crate::protocol_serde::shape_contact_channel_address::ser_contact_channel_address(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    Ok(())
}
