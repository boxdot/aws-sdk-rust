// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartConnectionInput {
    /// <p> The serial number of the dongle. </p>
    #[doc(hidden)]
    pub device_serial_number: ::std::option::Option<::std::string::String>,
    /// <p> The ID of the Outpost server. </p>
    #[doc(hidden)]
    pub asset_id: ::std::option::Option<::std::string::String>,
    /// <p> The public key of the client. </p>
    #[doc(hidden)]
    pub client_public_key: ::std::option::Option<::std::string::String>,
    /// <p> The device index of the network interface on the Outpost server. </p>
    #[doc(hidden)]
    pub network_interface_device_index: ::std::option::Option<i32>,
}
impl StartConnectionInput {
    /// <p> The serial number of the dongle. </p>
    pub fn device_serial_number(&self) -> ::std::option::Option<&str> {
        self.device_serial_number.as_deref()
    }
    /// <p> The ID of the Outpost server. </p>
    pub fn asset_id(&self) -> ::std::option::Option<&str> {
        self.asset_id.as_deref()
    }
    /// <p> The public key of the client. </p>
    pub fn client_public_key(&self) -> ::std::option::Option<&str> {
        self.client_public_key.as_deref()
    }
    /// <p> The device index of the network interface on the Outpost server. </p>
    pub fn network_interface_device_index(&self) -> ::std::option::Option<i32> {
        self.network_interface_device_index
    }
}
impl StartConnectionInput {
    /// Creates a new builder-style object to manufacture [`StartConnectionInput`](crate::operation::start_connection::StartConnectionInput).
    pub fn builder() -> crate::operation::start_connection::builders::StartConnectionInputBuilder {
        crate::operation::start_connection::builders::StartConnectionInputBuilder::default()
    }
}

/// A builder for [`StartConnectionInput`](crate::operation::start_connection::StartConnectionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartConnectionInputBuilder {
    pub(crate) device_serial_number: ::std::option::Option<::std::string::String>,
    pub(crate) asset_id: ::std::option::Option<::std::string::String>,
    pub(crate) client_public_key: ::std::option::Option<::std::string::String>,
    pub(crate) network_interface_device_index: ::std::option::Option<i32>,
}
impl StartConnectionInputBuilder {
    /// <p> The serial number of the dongle. </p>
    pub fn device_serial_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.device_serial_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The serial number of the dongle. </p>
    pub fn set_device_serial_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.device_serial_number = input;
        self
    }
    /// <p> The ID of the Outpost server. </p>
    pub fn asset_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asset_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ID of the Outpost server. </p>
    pub fn set_asset_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asset_id = input;
        self
    }
    /// <p> The public key of the client. </p>
    pub fn client_public_key(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.client_public_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The public key of the client. </p>
    pub fn set_client_public_key(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.client_public_key = input;
        self
    }
    /// <p> The device index of the network interface on the Outpost server. </p>
    pub fn network_interface_device_index(mut self, input: i32) -> Self {
        self.network_interface_device_index = ::std::option::Option::Some(input);
        self
    }
    /// <p> The device index of the network interface on the Outpost server. </p>
    pub fn set_network_interface_device_index(mut self, input: ::std::option::Option<i32>) -> Self {
        self.network_interface_device_index = input;
        self
    }
    /// Consumes the builder and constructs a [`StartConnectionInput`](crate::operation::start_connection::StartConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_connection::StartConnectionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_connection::StartConnectionInput {
            device_serial_number: self.device_serial_number,
            asset_id: self.asset_id,
            client_public_key: self.client_public_key,
            network_interface_device_index: self.network_interface_device_index,
        })
    }
}
