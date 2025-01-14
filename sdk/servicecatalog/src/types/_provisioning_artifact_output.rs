// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provisioning artifact output.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvisioningArtifactOutput {
    /// <p>The provisioning artifact output key.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>Description of the provisioning artifact output key.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
}
impl ProvisioningArtifactOutput {
    /// <p>The provisioning artifact output key.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>Description of the provisioning artifact output key.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl ProvisioningArtifactOutput {
    /// Creates a new builder-style object to manufacture [`ProvisioningArtifactOutput`](crate::types::ProvisioningArtifactOutput).
    pub fn builder() -> crate::types::builders::ProvisioningArtifactOutputBuilder {
        crate::types::builders::ProvisioningArtifactOutputBuilder::default()
    }
}

/// A builder for [`ProvisioningArtifactOutput`](crate::types::ProvisioningArtifactOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProvisioningArtifactOutputBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl ProvisioningArtifactOutputBuilder {
    /// <p>The provisioning artifact output key.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The provisioning artifact output key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>Description of the provisioning artifact output key.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Description of the provisioning artifact output key.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`ProvisioningArtifactOutput`](crate::types::ProvisioningArtifactOutput).
    pub fn build(self) -> crate::types::ProvisioningArtifactOutput {
        crate::types::ProvisioningArtifactOutput {
            key: self.key,
            description: self.description,
        }
    }
}
