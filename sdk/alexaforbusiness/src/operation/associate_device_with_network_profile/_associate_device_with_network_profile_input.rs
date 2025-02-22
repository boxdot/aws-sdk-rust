// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateDeviceWithNetworkProfileInput {
    /// <p>The device ARN.</p>
    #[doc(hidden)]
    pub device_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the network profile to associate with a device.</p>
    #[doc(hidden)]
    pub network_profile_arn: ::std::option::Option<::std::string::String>,
}
impl AssociateDeviceWithNetworkProfileInput {
    /// <p>The device ARN.</p>
    pub fn device_arn(&self) -> ::std::option::Option<&str> {
        self.device_arn.as_deref()
    }
    /// <p>The ARN of the network profile to associate with a device.</p>
    pub fn network_profile_arn(&self) -> ::std::option::Option<&str> {
        self.network_profile_arn.as_deref()
    }
}
impl AssociateDeviceWithNetworkProfileInput {
    /// Creates a new builder-style object to manufacture [`AssociateDeviceWithNetworkProfileInput`](crate::operation::associate_device_with_network_profile::AssociateDeviceWithNetworkProfileInput).
    pub fn builder() -> crate::operation::associate_device_with_network_profile::builders::AssociateDeviceWithNetworkProfileInputBuilder{
        crate::operation::associate_device_with_network_profile::builders::AssociateDeviceWithNetworkProfileInputBuilder::default()
    }
}

/// A builder for [`AssociateDeviceWithNetworkProfileInput`](crate::operation::associate_device_with_network_profile::AssociateDeviceWithNetworkProfileInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociateDeviceWithNetworkProfileInputBuilder {
    pub(crate) device_arn: ::std::option::Option<::std::string::String>,
    pub(crate) network_profile_arn: ::std::option::Option<::std::string::String>,
}
impl AssociateDeviceWithNetworkProfileInputBuilder {
    /// <p>The device ARN.</p>
    pub fn device_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The device ARN.</p>
    pub fn set_device_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_arn = input;
        self
    }
    /// <p>The ARN of the network profile to associate with a device.</p>
    pub fn network_profile_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.network_profile_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the network profile to associate with a device.</p>
    pub fn set_network_profile_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.network_profile_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`AssociateDeviceWithNetworkProfileInput`](crate::operation::associate_device_with_network_profile::AssociateDeviceWithNetworkProfileInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::associate_device_with_network_profile::AssociateDeviceWithNetworkProfileInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::associate_device_with_network_profile::AssociateDeviceWithNetworkProfileInput {
                device_arn: self.device_arn
                ,
                network_profile_arn: self.network_profile_arn
                ,
            }
        )
    }
}
