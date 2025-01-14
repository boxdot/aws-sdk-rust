// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cidr_collection_change(
    input: &crate::types::CidrCollectionChange,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.location_name {
        let mut inner_writer = scope.start_el("LocationName").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.action {
        let mut inner_writer = scope.start_el("Action").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.cidr_list {
        let mut inner_writer = scope.start_el("CidrList").finish();
        for list_item_4 in var_3 {
            {
                let mut inner_writer = inner_writer.start_el("Cidr").finish();
                inner_writer.data(list_item_4.as_str());
            }
        }
    }
    scope.finish();
    Ok(())
}
