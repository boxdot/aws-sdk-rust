// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An IP address/port combination.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SocketAddress {
    /// <p>The IP address for the socket address.</p>
    #[doc(hidden)]
    pub ip_address: ::std::option::Option<::std::string::String>,
    /// <p>The port for the socket address.</p>
    #[doc(hidden)]
    pub port: ::std::option::Option<i32>,
}
impl SocketAddress {
    /// <p>The IP address for the socket address.</p>
    pub fn ip_address(&self) -> ::std::option::Option<&str> {
        self.ip_address.as_deref()
    }
    /// <p>The port for the socket address.</p>
    pub fn port(&self) -> ::std::option::Option<i32> {
        self.port
    }
}
impl SocketAddress {
    /// Creates a new builder-style object to manufacture [`SocketAddress`](crate::types::SocketAddress).
    pub fn builder() -> crate::types::builders::SocketAddressBuilder {
        crate::types::builders::SocketAddressBuilder::default()
    }
}

/// A builder for [`SocketAddress`](crate::types::SocketAddress).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SocketAddressBuilder {
    pub(crate) ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) port: ::std::option::Option<i32>,
}
impl SocketAddressBuilder {
    /// <p>The IP address for the socket address.</p>
    pub fn ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address for the socket address.</p>
    pub fn set_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_address = input;
        self
    }
    /// <p>The port for the socket address.</p>
    pub fn port(mut self, input: i32) -> Self {
        self.port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The port for the socket address.</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.port = input;
        self
    }
    /// Consumes the builder and constructs a [`SocketAddress`](crate::types::SocketAddress).
    pub fn build(self) -> crate::types::SocketAddress {
        crate::types::SocketAddress {
            ip_address: self.ip_address,
            port: self.port,
        }
    }
}
