// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_click_feedback(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ClickFeedback,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.result_id {
        object.key("ResultId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.click_time {
        object
            .key("ClickTime")
            .date_time(var_2, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}
