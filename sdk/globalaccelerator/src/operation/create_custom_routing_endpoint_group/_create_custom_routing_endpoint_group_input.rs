// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateCustomRoutingEndpointGroupInput {
    /// <p>The Amazon Resource Name (ARN) of the listener for a custom routing endpoint.</p>
    #[doc(hidden)]
    pub listener_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a specific Region.</p>
    #[doc(hidden)]
    pub endpoint_group_region: ::std::option::Option<::std::string::String>,
    /// <p>Sets the port range and protocol for all endpoints (virtual private cloud subnets) in a custom routing endpoint group to accept client traffic on.</p>
    #[doc(hidden)]
    pub destination_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::CustomRoutingDestinationConfiguration>>,
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    #[doc(hidden)]
    pub idempotency_token: ::std::option::Option<::std::string::String>,
}
impl CreateCustomRoutingEndpointGroupInput {
    /// <p>The Amazon Resource Name (ARN) of the listener for a custom routing endpoint.</p>
    pub fn listener_arn(&self) -> ::std::option::Option<&str> {
        self.listener_arn.as_deref()
    }
    /// <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a specific Region.</p>
    pub fn endpoint_group_region(&self) -> ::std::option::Option<&str> {
        self.endpoint_group_region.as_deref()
    }
    /// <p>Sets the port range and protocol for all endpoints (virtual private cloud subnets) in a custom routing endpoint group to accept client traffic on.</p>
    pub fn destination_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::CustomRoutingDestinationConfiguration]> {
        self.destination_configurations.as_deref()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn idempotency_token(&self) -> ::std::option::Option<&str> {
        self.idempotency_token.as_deref()
    }
}
impl CreateCustomRoutingEndpointGroupInput {
    /// Creates a new builder-style object to manufacture [`CreateCustomRoutingEndpointGroupInput`](crate::operation::create_custom_routing_endpoint_group::CreateCustomRoutingEndpointGroupInput).
    pub fn builder() -> crate::operation::create_custom_routing_endpoint_group::builders::CreateCustomRoutingEndpointGroupInputBuilder{
        crate::operation::create_custom_routing_endpoint_group::builders::CreateCustomRoutingEndpointGroupInputBuilder::default()
    }
}

/// A builder for [`CreateCustomRoutingEndpointGroupInput`](crate::operation::create_custom_routing_endpoint_group::CreateCustomRoutingEndpointGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateCustomRoutingEndpointGroupInputBuilder {
    pub(crate) listener_arn: ::std::option::Option<::std::string::String>,
    pub(crate) endpoint_group_region: ::std::option::Option<::std::string::String>,
    pub(crate) destination_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::CustomRoutingDestinationConfiguration>>,
    pub(crate) idempotency_token: ::std::option::Option<::std::string::String>,
}
impl CreateCustomRoutingEndpointGroupInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the listener for a custom routing endpoint.</p>
    pub fn listener_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.listener_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the listener for a custom routing endpoint.</p>
    pub fn set_listener_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.listener_arn = input;
        self
    }
    /// <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a specific Region.</p>
    pub fn endpoint_group_region(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.endpoint_group_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region where the endpoint group is located. A listener can have only one endpoint group in a specific Region.</p>
    pub fn set_endpoint_group_region(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.endpoint_group_region = input;
        self
    }
    /// Appends an item to `destination_configurations`.
    ///
    /// To override the contents of this collection use [`set_destination_configurations`](Self::set_destination_configurations).
    ///
    /// <p>Sets the port range and protocol for all endpoints (virtual private cloud subnets) in a custom routing endpoint group to accept client traffic on.</p>
    pub fn destination_configurations(
        mut self,
        input: crate::types::CustomRoutingDestinationConfiguration,
    ) -> Self {
        let mut v = self.destination_configurations.unwrap_or_default();
        v.push(input);
        self.destination_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Sets the port range and protocol for all endpoints (virtual private cloud subnets) in a custom routing endpoint group to accept client traffic on.</p>
    pub fn set_destination_configurations(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::CustomRoutingDestinationConfiguration>,
        >,
    ) -> Self {
        self.destination_configurations = input;
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn idempotency_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.idempotency_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn set_idempotency_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.idempotency_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateCustomRoutingEndpointGroupInput`](crate::operation::create_custom_routing_endpoint_group::CreateCustomRoutingEndpointGroupInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_custom_routing_endpoint_group::CreateCustomRoutingEndpointGroupInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::create_custom_routing_endpoint_group::CreateCustomRoutingEndpointGroupInput {
                listener_arn: self.listener_arn
                ,
                endpoint_group_region: self.endpoint_group_region
                ,
                destination_configurations: self.destination_configurations
                ,
                idempotency_token: self.idempotency_token
                ,
            }
        )
    }
}
