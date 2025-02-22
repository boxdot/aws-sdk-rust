// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> The possible Amazon Web Services Free Tier configurations. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateFreeTierConfig {
    /// <p> Activate or deactivate application of Amazon Web Services Free Tier. </p>
    #[doc(hidden)]
    pub activated: ::std::option::Option<bool>,
}
impl UpdateFreeTierConfig {
    /// <p> Activate or deactivate application of Amazon Web Services Free Tier. </p>
    pub fn activated(&self) -> ::std::option::Option<bool> {
        self.activated
    }
}
impl UpdateFreeTierConfig {
    /// Creates a new builder-style object to manufacture [`UpdateFreeTierConfig`](crate::types::UpdateFreeTierConfig).
    pub fn builder() -> crate::types::builders::UpdateFreeTierConfigBuilder {
        crate::types::builders::UpdateFreeTierConfigBuilder::default()
    }
}

/// A builder for [`UpdateFreeTierConfig`](crate::types::UpdateFreeTierConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateFreeTierConfigBuilder {
    pub(crate) activated: ::std::option::Option<bool>,
}
impl UpdateFreeTierConfigBuilder {
    /// <p> Activate or deactivate application of Amazon Web Services Free Tier. </p>
    pub fn activated(mut self, input: bool) -> Self {
        self.activated = ::std::option::Option::Some(input);
        self
    }
    /// <p> Activate or deactivate application of Amazon Web Services Free Tier. </p>
    pub fn set_activated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.activated = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateFreeTierConfig`](crate::types::UpdateFreeTierConfig).
    pub fn build(self) -> crate::types::UpdateFreeTierConfig {
        crate::types::UpdateFreeTierConfig {
            activated: self.activated,
        }
    }
}
