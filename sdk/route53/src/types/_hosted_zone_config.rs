// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type that contains an optional comment about your hosted zone. If you don't want to specify a comment, omit both the <code>HostedZoneConfig</code> and <code>Comment</code> elements.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HostedZoneConfig {
    /// <p>Any comments that you want to include about the hosted zone.</p>
    #[doc(hidden)]
    pub comment: ::std::option::Option<::std::string::String>,
    /// <p>A value that indicates whether this is a private hosted zone.</p>
    #[doc(hidden)]
    pub private_zone: bool,
}
impl HostedZoneConfig {
    /// <p>Any comments that you want to include about the hosted zone.</p>
    pub fn comment(&self) -> ::std::option::Option<&str> {
        self.comment.as_deref()
    }
    /// <p>A value that indicates whether this is a private hosted zone.</p>
    pub fn private_zone(&self) -> bool {
        self.private_zone
    }
}
impl HostedZoneConfig {
    /// Creates a new builder-style object to manufacture [`HostedZoneConfig`](crate::types::HostedZoneConfig).
    pub fn builder() -> crate::types::builders::HostedZoneConfigBuilder {
        crate::types::builders::HostedZoneConfigBuilder::default()
    }
}

/// A builder for [`HostedZoneConfig`](crate::types::HostedZoneConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HostedZoneConfigBuilder {
    pub(crate) comment: ::std::option::Option<::std::string::String>,
    pub(crate) private_zone: ::std::option::Option<bool>,
}
impl HostedZoneConfigBuilder {
    /// <p>Any comments that you want to include about the hosted zone.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Any comments that you want to include about the hosted zone.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comment = input;
        self
    }
    /// <p>A value that indicates whether this is a private hosted zone.</p>
    pub fn private_zone(mut self, input: bool) -> Self {
        self.private_zone = ::std::option::Option::Some(input);
        self
    }
    /// <p>A value that indicates whether this is a private hosted zone.</p>
    pub fn set_private_zone(mut self, input: ::std::option::Option<bool>) -> Self {
        self.private_zone = input;
        self
    }
    /// Consumes the builder and constructs a [`HostedZoneConfig`](crate::types::HostedZoneConfig).
    pub fn build(self) -> crate::types::HostedZoneConfig {
        crate::types::HostedZoneConfig {
            comment: self.comment,
            private_zone: self.private_zone.unwrap_or_default(),
        }
    }
}
