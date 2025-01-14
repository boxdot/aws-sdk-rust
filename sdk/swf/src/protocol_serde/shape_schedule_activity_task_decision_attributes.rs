// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_schedule_activity_task_decision_attributes(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ScheduleActivityTaskDecisionAttributes,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.activity_type {
        #[allow(unused_mut)]
        let mut object_2 = object.key("activityType").start_object();
        crate::protocol_serde::shape_activity_type::ser_activity_type(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.activity_id {
        object.key("activityId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.control {
        object.key("control").string(var_4.as_str());
    }
    if let Some(var_5) = &input.input {
        object.key("input").string(var_5.as_str());
    }
    if let Some(var_6) = &input.schedule_to_close_timeout {
        object.key("scheduleToCloseTimeout").string(var_6.as_str());
    }
    if let Some(var_7) = &input.task_list {
        #[allow(unused_mut)]
        let mut object_8 = object.key("taskList").start_object();
        crate::protocol_serde::shape_task_list::ser_task_list(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.task_priority {
        object.key("taskPriority").string(var_9.as_str());
    }
    if let Some(var_10) = &input.schedule_to_start_timeout {
        object.key("scheduleToStartTimeout").string(var_10.as_str());
    }
    if let Some(var_11) = &input.start_to_close_timeout {
        object.key("startToCloseTimeout").string(var_11.as_str());
    }
    if let Some(var_12) = &input.heartbeat_timeout {
        object.key("heartbeatTimeout").string(var_12.as_str());
    }
    Ok(())
}
