// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetWirelessGatewayInput {
    /// <p>The identifier of the wireless gateway to get.</p>
    #[doc(hidden)]
    pub identifier: ::std::option::Option<::std::string::String>,
    /// <p>The type of identifier used in <code>identifier</code>.</p>
    #[doc(hidden)]
    pub identifier_type: ::std::option::Option<crate::types::WirelessGatewayIdType>,
}
impl GetWirelessGatewayInput {
    /// <p>The identifier of the wireless gateway to get.</p>
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }
    /// <p>The type of identifier used in <code>identifier</code>.</p>
    pub fn identifier_type(&self) -> ::std::option::Option<&crate::types::WirelessGatewayIdType> {
        self.identifier_type.as_ref()
    }
}
impl GetWirelessGatewayInput {
    /// Creates a new builder-style object to manufacture [`GetWirelessGatewayInput`](crate::operation::get_wireless_gateway::GetWirelessGatewayInput).
    pub fn builder(
    ) -> crate::operation::get_wireless_gateway::builders::GetWirelessGatewayInputBuilder {
        crate::operation::get_wireless_gateway::builders::GetWirelessGatewayInputBuilder::default()
    }
}

/// A builder for [`GetWirelessGatewayInput`](crate::operation::get_wireless_gateway::GetWirelessGatewayInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetWirelessGatewayInputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
    pub(crate) identifier_type: ::std::option::Option<crate::types::WirelessGatewayIdType>,
}
impl GetWirelessGatewayInputBuilder {
    /// <p>The identifier of the wireless gateway to get.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the wireless gateway to get.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }
    /// <p>The type of identifier used in <code>identifier</code>.</p>
    pub fn identifier_type(mut self, input: crate::types::WirelessGatewayIdType) -> Self {
        self.identifier_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of identifier used in <code>identifier</code>.</p>
    pub fn set_identifier_type(
        mut self,
        input: ::std::option::Option<crate::types::WirelessGatewayIdType>,
    ) -> Self {
        self.identifier_type = input;
        self
    }
    /// Consumes the builder and constructs a [`GetWirelessGatewayInput`](crate::operation::get_wireless_gateway::GetWirelessGatewayInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_wireless_gateway::GetWirelessGatewayInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_wireless_gateway::GetWirelessGatewayInput {
                identifier: self.identifier,
                identifier_type: self.identifier_type,
            },
        )
    }
}
