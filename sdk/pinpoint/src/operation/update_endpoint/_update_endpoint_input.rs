// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateEndpointInput {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the endpoint.</p>
    #[doc(hidden)]
    pub endpoint_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the channel type and other settings for an endpoint.</p>
    #[doc(hidden)]
    pub endpoint_request: ::std::option::Option<crate::types::EndpointRequest>,
}
impl UpdateEndpointInput {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The unique identifier for the endpoint.</p>
    pub fn endpoint_id(&self) -> ::std::option::Option<&str> {
        self.endpoint_id.as_deref()
    }
    /// <p>Specifies the channel type and other settings for an endpoint.</p>
    pub fn endpoint_request(&self) -> ::std::option::Option<&crate::types::EndpointRequest> {
        self.endpoint_request.as_ref()
    }
}
impl UpdateEndpointInput {
    /// Creates a new builder-style object to manufacture [`UpdateEndpointInput`](crate::operation::update_endpoint::UpdateEndpointInput).
    pub fn builder() -> crate::operation::update_endpoint::builders::UpdateEndpointInputBuilder {
        crate::operation::update_endpoint::builders::UpdateEndpointInputBuilder::default()
    }
}

/// A builder for [`UpdateEndpointInput`](crate::operation::update_endpoint::UpdateEndpointInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateEndpointInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) endpoint_id: ::std::option::Option<::std::string::String>,
    pub(crate) endpoint_request: ::std::option::Option<crate::types::EndpointRequest>,
}
impl UpdateEndpointInputBuilder {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The unique identifier for the endpoint.</p>
    pub fn endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the endpoint.</p>
    pub fn set_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint_id = input;
        self
    }
    /// <p>Specifies the channel type and other settings for an endpoint.</p>
    pub fn endpoint_request(mut self, input: crate::types::EndpointRequest) -> Self {
        self.endpoint_request = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the channel type and other settings for an endpoint.</p>
    pub fn set_endpoint_request(
        mut self,
        input: ::std::option::Option<crate::types::EndpointRequest>,
    ) -> Self {
        self.endpoint_request = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateEndpointInput`](crate::operation::update_endpoint::UpdateEndpointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_endpoint::UpdateEndpointInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_endpoint::UpdateEndpointInput {
            application_id: self.application_id,
            endpoint_id: self.endpoint_id,
            endpoint_request: self.endpoint_request,
        })
    }
}
