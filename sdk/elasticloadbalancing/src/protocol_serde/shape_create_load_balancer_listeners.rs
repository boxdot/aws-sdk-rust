// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_load_balancer_listeners_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersOutput,
    crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::unhandled(generic))
                            };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "LoadBalancerNotFound" => crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::AccessPointNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessPointNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_point_not_found_exception::de_access_point_not_found_exception_xml_err(_response_body, output).map_err(crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CertificateNotFound" => crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::CertificateNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CertificateNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_certificate_not_found_exception::de_certificate_not_found_exception_xml_err(_response_body, output).map_err(crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DuplicateListener" => crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::DuplicateListenerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DuplicateListenerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_duplicate_listener_exception::de_duplicate_listener_exception_xml_err(_response_body, output).map_err(crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidConfigurationRequest" => crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::InvalidConfigurationRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidConfigurationRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_configuration_request_exception::de_invalid_configuration_request_exception_xml_err(_response_body, output).map_err(crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedProtocol" => crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::UnsupportedProtocolException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedProtocolExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unsupported_protocol_exception::de_unsupported_protocol_exception_xml_err(_response_body, output).map_err(crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_load_balancer_listeners_http_response_with_props(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersOutput,
    crate::operation::create_load_balancer_listeners::CreateLoadBalancerListenersError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_load_balancer_listeners::builders::CreateLoadBalancerListenersOutputBuilder::default();
        output._set_request_id(
            ::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string),
        );
        output.build()
    })
}
