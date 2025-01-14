// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sheet_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SheetDefinition,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sheet_id {
        object.key("SheetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.title {
        object.key("Title").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("Name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.parameter_controls {
        let mut array_6 = object.key("ParameterControls").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_parameter_control::ser_parameter_control(
                    &mut object_8,
                    item_7,
                )?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.filter_controls {
        let mut array_10 = object.key("FilterControls").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_filter_control::ser_filter_control(
                    &mut object_12,
                    item_11,
                )?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.visuals {
        let mut array_14 = object.key("Visuals").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_visual::ser_visual(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.text_boxes {
        let mut array_18 = object.key("TextBoxes").start_array();
        for item_19 in var_17 {
            {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_sheet_text_box::ser_sheet_text_box(
                    &mut object_20,
                    item_19,
                )?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.layouts {
        let mut array_22 = object.key("Layouts").start_array();
        for item_23 in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::protocol_serde::shape_layout::ser_layout(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.sheet_control_layouts {
        let mut array_26 = object.key("SheetControlLayouts").start_array();
        for item_27 in var_25 {
            {
                #[allow(unused_mut)]
                let mut object_28 = array_26.value().start_object();
                crate::protocol_serde::shape_sheet_control_layout::ser_sheet_control_layout(
                    &mut object_28,
                    item_27,
                )?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    if let Some(var_29) = &input.content_type {
        object.key("ContentType").string(var_29.as_str());
    }
    Ok(())
}

pub(crate) fn de_sheet_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::SheetDefinition>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            ::aws_smithy_json::deserialize::Token<'a>,
            ::aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SheetDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key
                        .to_unescaped()?
                        .as_ref()
                    {
                        "SheetId" => {
                            builder = builder.set_sheet_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "Title" => {
                            builder = builder.set_title(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?,
                            );
                        }
                        "ParameterControls" => {
                            builder = builder.set_parameter_controls(
                                    crate::protocol_serde::shape_parameter_control_list::de_parameter_control_list(tokens)?
                                );
                        }
                        "FilterControls" => {
                            builder = builder.set_filter_controls(
                                    crate::protocol_serde::shape_filter_control_list::de_filter_control_list(tokens)?
                                );
                        }
                        "Visuals" => {
                            builder = builder.set_visuals(
                                crate::protocol_serde::shape_visual_list::de_visual_list(tokens)?,
                            );
                        }
                        "TextBoxes" => {
                            builder = builder.set_text_boxes(
                                    crate::protocol_serde::shape_sheet_text_box_list::de_sheet_text_box_list(tokens)?
                                );
                        }
                        "Layouts" => {
                            builder = builder.set_layouts(
                                crate::protocol_serde::shape_layout_list::de_layout_list(tokens)?,
                            );
                        }
                        "SheetControlLayouts" => {
                            builder = builder.set_sheet_control_layouts(
                                    crate::protocol_serde::shape_sheet_control_layout_list::de_sheet_control_layout_list(tokens)?
                                );
                        }
                        "ContentType" => {
                            builder = builder.set_content_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(
                                    tokens.next(),
                                )?
                                .map(|s| {
                                    s.to_unescaped()
                                        .map(|u| crate::types::SheetContentType::from(u.as_ref()))
                                })
                                .transpose()?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(
                            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                format!("expected object key or end object, found: {:?}", other),
                            ),
                        )
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(
            ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}
