// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_response_headers_policy_config(
    input: &crate::types::ResponseHeadersPolicyConfig,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.comment {
        let mut inner_writer = scope.start_el("Comment").finish();
        inner_writer.data(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        let mut inner_writer = scope.start_el("Name").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.cors_config {
        let inner_writer = scope.start_el("CorsConfig");
        crate::protocol_serde::shape_response_headers_policy_cors_config::ser_response_headers_policy_cors_config(var_3, inner_writer)?
    }
    if let Some(var_4) = &input.security_headers_config {
        let inner_writer = scope.start_el("SecurityHeadersConfig");
        crate::protocol_serde::shape_response_headers_policy_security_headers_config::ser_response_headers_policy_security_headers_config(var_4, inner_writer)?
    }
    if let Some(var_5) = &input.server_timing_headers_config {
        let inner_writer = scope.start_el("ServerTimingHeadersConfig");
        crate::protocol_serde::shape_response_headers_policy_server_timing_headers_config::ser_response_headers_policy_server_timing_headers_config(var_5, inner_writer)?
    }
    if let Some(var_6) = &input.custom_headers_config {
        let inner_writer = scope.start_el("CustomHeadersConfig");
        crate::protocol_serde::shape_response_headers_policy_custom_headers_config::ser_response_headers_policy_custom_headers_config(var_6, inner_writer)?
    }
    if let Some(var_7) = &input.remove_headers_config {
        let inner_writer = scope.start_el("RemoveHeadersConfig");
        crate::protocol_serde::shape_response_headers_policy_remove_headers_config::ser_response_headers_policy_remove_headers_config(var_7, inner_writer)?
    }
    scope.finish();
    Ok(())
}

pub fn de_response_headers_policy_config(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ResponseHeadersPolicyConfig, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ResponseHeadersPolicyConfig::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Comment") /* Comment com.amazonaws.cloudfront#ResponseHeadersPolicyConfig$Comment */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_comment(var_8);
            }
            ,
            s if s.matches("Name") /* Name com.amazonaws.cloudfront#ResponseHeadersPolicyConfig$Name */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_9);
            }
            ,
            s if s.matches("CorsConfig") /* CorsConfig com.amazonaws.cloudfront#ResponseHeadersPolicyConfig$CorsConfig */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_response_headers_policy_cors_config::de_response_headers_policy_cors_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cors_config(var_10);
            }
            ,
            s if s.matches("SecurityHeadersConfig") /* SecurityHeadersConfig com.amazonaws.cloudfront#ResponseHeadersPolicyConfig$SecurityHeadersConfig */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_response_headers_policy_security_headers_config::de_response_headers_policy_security_headers_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_headers_config(var_11);
            }
            ,
            s if s.matches("ServerTimingHeadersConfig") /* ServerTimingHeadersConfig com.amazonaws.cloudfront#ResponseHeadersPolicyConfig$ServerTimingHeadersConfig */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_response_headers_policy_server_timing_headers_config::de_response_headers_policy_server_timing_headers_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_server_timing_headers_config(var_12);
            }
            ,
            s if s.matches("CustomHeadersConfig") /* CustomHeadersConfig com.amazonaws.cloudfront#ResponseHeadersPolicyConfig$CustomHeadersConfig */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_response_headers_policy_custom_headers_config::de_response_headers_policy_custom_headers_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_custom_headers_config(var_13);
            }
            ,
            s if s.matches("RemoveHeadersConfig") /* RemoveHeadersConfig com.amazonaws.cloudfront#ResponseHeadersPolicyConfig$RemoveHeadersConfig */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_response_headers_policy_remove_headers_config::de_response_headers_policy_remove_headers_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_remove_headers_config(var_14);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
