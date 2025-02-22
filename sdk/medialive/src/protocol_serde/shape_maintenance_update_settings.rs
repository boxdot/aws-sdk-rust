// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_maintenance_update_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MaintenanceUpdateSettings,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.maintenance_day {
        object.key("maintenanceDay").string(var_1.as_str());
    }
    if let Some(var_2) = &input.maintenance_scheduled_date {
        object
            .key("maintenanceScheduledDate")
            .string(var_2.as_str());
    }
    if let Some(var_3) = &input.maintenance_start_time {
        object.key("maintenanceStartTime").string(var_3.as_str());
    }
    Ok(())
}
