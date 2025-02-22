// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_template_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_template::UpdateTemplateInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.layout_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("layoutConfiguration").start_object();
        crate::protocol_serde::shape_layout_configuration::ser_layout_configuration(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.name {
        object.key("name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.required_fields {
        let mut array_6 = object.key("requiredFields").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_required_field::ser_required_field(
                    &mut object_8,
                    item_7,
                )?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.status {
        object.key("status").string(var_9.as_str());
    }
    Ok(())
}
