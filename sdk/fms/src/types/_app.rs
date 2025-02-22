// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An individual Firewall Manager application.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct App {
    /// <p>The application's name.</p>
    #[doc(hidden)]
    pub app_name: ::std::option::Option<::std::string::String>,
    /// <p>The IP protocol name or number. The name can be one of <code>tcp</code>, <code>udp</code>, or <code>icmp</code>. For information on possible numbers, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>.</p>
    #[doc(hidden)]
    pub protocol: ::std::option::Option<::std::string::String>,
    /// <p>The application's port number, for example <code>80</code>.</p>
    #[doc(hidden)]
    pub port: ::std::option::Option<i64>,
}
impl App {
    /// <p>The application's name.</p>
    pub fn app_name(&self) -> ::std::option::Option<&str> {
        self.app_name.as_deref()
    }
    /// <p>The IP protocol name or number. The name can be one of <code>tcp</code>, <code>udp</code>, or <code>icmp</code>. For information on possible numbers, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>.</p>
    pub fn protocol(&self) -> ::std::option::Option<&str> {
        self.protocol.as_deref()
    }
    /// <p>The application's port number, for example <code>80</code>.</p>
    pub fn port(&self) -> ::std::option::Option<i64> {
        self.port
    }
}
impl App {
    /// Creates a new builder-style object to manufacture [`App`](crate::types::App).
    pub fn builder() -> crate::types::builders::AppBuilder {
        crate::types::builders::AppBuilder::default()
    }
}

/// A builder for [`App`](crate::types::App).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AppBuilder {
    pub(crate) app_name: ::std::option::Option<::std::string::String>,
    pub(crate) protocol: ::std::option::Option<::std::string::String>,
    pub(crate) port: ::std::option::Option<i64>,
}
impl AppBuilder {
    /// <p>The application's name.</p>
    pub fn app_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The application's name.</p>
    pub fn set_app_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_name = input;
        self
    }
    /// <p>The IP protocol name or number. The name can be one of <code>tcp</code>, <code>udp</code>, or <code>icmp</code>. For information on possible numbers, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>.</p>
    pub fn protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP protocol name or number. The name can be one of <code>tcp</code>, <code>udp</code>, or <code>icmp</code>. For information on possible numbers, see <a href="https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The application's port number, for example <code>80</code>.</p>
    pub fn port(mut self, input: i64) -> Self {
        self.port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The application's port number, for example <code>80</code>.</p>
    pub fn set_port(mut self, input: ::std::option::Option<i64>) -> Self {
        self.port = input;
        self
    }
    /// Consumes the builder and constructs a [`App`](crate::types::App).
    pub fn build(self) -> crate::types::App {
        crate::types::App {
            app_name: self.app_name,
            protocol: self.protocol,
            port: self.port,
        }
    }
}
