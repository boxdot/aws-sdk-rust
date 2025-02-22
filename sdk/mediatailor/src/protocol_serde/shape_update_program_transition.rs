// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_program_transition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateProgramTransition,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if input.scheduled_start_time_millis != 0 {
        object.key("ScheduledStartTimeMillis").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.scheduled_start_time_millis).into()),
        );
    }
    if input.duration_millis != 0 {
        object.key("DurationMillis").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.duration_millis).into()),
        );
    }
    Ok(())
}
