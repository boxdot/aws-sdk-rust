// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_metric_query(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MetricQuery,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.metric {
        object.key("Metric").string(var_1.as_str());
    }
    if let Some(var_2) = &input.group_by {
        #[allow(unused_mut)]
        let mut object_3 = object.key("GroupBy").start_object();
        crate::protocol_serde::shape_dimension_group::ser_dimension_group(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.filter {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Filter").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}
