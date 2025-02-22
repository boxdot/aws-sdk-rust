// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_registered_user_dashboard_feature_configurations(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RegisteredUserDashboardFeatureConfigurations,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.state_persistence {
        #[allow(unused_mut)]
        let mut object_2 = object.key("StatePersistence").start_object();
        crate::protocol_serde::shape_state_persistence_configurations::ser_state_persistence_configurations(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.bookmarks {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Bookmarks").start_object();
        crate::protocol_serde::shape_bookmarks_configurations::ser_bookmarks_configurations(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    Ok(())
}
