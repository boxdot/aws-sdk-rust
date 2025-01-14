// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_through_resources_statement_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ThroughResourcesStatementRequest,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ResourceStatement");
    if let Some(var_2) = &input.resource_statement {
        crate::protocol_serde::shape_resource_statement_request::ser_resource_statement_request(
            scope_1, var_2,
        )?;
    }
    Ok(())
}
