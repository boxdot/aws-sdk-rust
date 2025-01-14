// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_lambda_function_configuration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LambdaFunctionConfiguration, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LambdaFunctionConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.s3#LambdaFunctionConfiguration$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("CloudFunction") /* LambdaFunctionArn com.amazonaws.s3#LambdaFunctionConfiguration$LambdaFunctionArn */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_lambda_function_arn(var_2);
            }
            ,
            s if s.matches("Event") /* Events com.amazonaws.s3#LambdaFunctionConfiguration$Events */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::Event>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_4 = builder.events.take().unwrap_or_default();
                            list_4.push(
                                Result::<crate::types::Event, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                                    crate::types::Event::from(
                                        ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                                    )
                                )
                                ?
                            );
                            list_4
                        })
                        ?
                    )
                ;
                builder = builder.set_events(var_3);
            }
            ,
            s if s.matches("Filter") /* Filter com.amazonaws.s3#LambdaFunctionConfiguration$Filter */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_notification_configuration_filter::de_notification_configuration_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_filter(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_lambda_function_configuration(
    input: &crate::types::LambdaFunctionConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_6) = &input.id {
        let mut inner_writer = scope.start_el("Id").finish();
        inner_writer.data(var_6.as_str());
    }
    if let Some(var_7) = &input.lambda_function_arn {
        let mut inner_writer = scope.start_el("CloudFunction").finish();
        inner_writer.data(var_7.as_str());
    }
    if let Some(var_8) = &input.events {
        for list_item_9 in var_8 {
            {
                let mut inner_writer = scope.start_el("Event").finish();
                inner_writer.data(list_item_9.as_str());
            }
        }
    }
    if let Some(var_10) = &input.filter {
        let inner_writer = scope.start_el("Filter");
        crate::protocol_serde::shape_notification_configuration_filter::ser_notification_configuration_filter(var_10, inner_writer)?
    }
    scope.finish();
    Ok(())
}
