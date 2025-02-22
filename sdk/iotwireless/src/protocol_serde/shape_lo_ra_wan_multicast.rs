// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lo_ra_wan_multicast(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LoRaWanMulticast,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.rf_region {
        object.key("RfRegion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dl_class {
        object.key("DlClass").string(var_2.as_str());
    }
    Ok(())
}
