// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_attach_typed_link(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::BatchAttachTypedLink,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source_object_reference {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SourceObjectReference").start_object();
        crate::protocol_serde::shape_object_reference::ser_object_reference(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.target_object_reference {
        #[allow(unused_mut)]
        let mut object_4 = object.key("TargetObjectReference").start_object();
        crate::protocol_serde::shape_object_reference::ser_object_reference(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.typed_link_facet {
        #[allow(unused_mut)]
        let mut object_6 = object.key("TypedLinkFacet").start_object();
        crate::protocol_serde::shape_typed_link_schema_and_facet_name::ser_typed_link_schema_and_facet_name(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.attributes {
        let mut array_8 = object.key("Attributes").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_attribute_name_and_value::ser_attribute_name_and_value(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    Ok(())
}
