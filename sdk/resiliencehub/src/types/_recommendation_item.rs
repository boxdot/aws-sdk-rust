// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines a recommendation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecommendationItem {
    /// <p>The resource identifier.</p>
    #[doc(hidden)]
    pub resource_id: ::std::option::Option<::std::string::String>,
    /// <p>The target account identifier.</p>
    #[doc(hidden)]
    pub target_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The target region.</p>
    #[doc(hidden)]
    pub target_region: ::std::option::Option<::std::string::String>,
    /// <p>Specifies if the recommendation has already been implemented.</p>
    #[doc(hidden)]
    pub already_implemented: ::std::option::Option<bool>,
}
impl RecommendationItem {
    /// <p>The resource identifier.</p>
    pub fn resource_id(&self) -> ::std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The target account identifier.</p>
    pub fn target_account_id(&self) -> ::std::option::Option<&str> {
        self.target_account_id.as_deref()
    }
    /// <p>The target region.</p>
    pub fn target_region(&self) -> ::std::option::Option<&str> {
        self.target_region.as_deref()
    }
    /// <p>Specifies if the recommendation has already been implemented.</p>
    pub fn already_implemented(&self) -> ::std::option::Option<bool> {
        self.already_implemented
    }
}
impl RecommendationItem {
    /// Creates a new builder-style object to manufacture [`RecommendationItem`](crate::types::RecommendationItem).
    pub fn builder() -> crate::types::builders::RecommendationItemBuilder {
        crate::types::builders::RecommendationItemBuilder::default()
    }
}

/// A builder for [`RecommendationItem`](crate::types::RecommendationItem).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RecommendationItemBuilder {
    pub(crate) resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) target_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) target_region: ::std::option::Option<::std::string::String>,
    pub(crate) already_implemented: ::std::option::Option<bool>,
}
impl RecommendationItemBuilder {
    /// <p>The resource identifier.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The resource identifier.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The target account identifier.</p>
    pub fn target_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.target_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The target account identifier.</p>
    pub fn set_target_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.target_account_id = input;
        self
    }
    /// <p>The target region.</p>
    pub fn target_region(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.target_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The target region.</p>
    pub fn set_target_region(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.target_region = input;
        self
    }
    /// <p>Specifies if the recommendation has already been implemented.</p>
    pub fn already_implemented(mut self, input: bool) -> Self {
        self.already_implemented = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies if the recommendation has already been implemented.</p>
    pub fn set_already_implemented(mut self, input: ::std::option::Option<bool>) -> Self {
        self.already_implemented = input;
        self
    }
    /// Consumes the builder and constructs a [`RecommendationItem`](crate::types::RecommendationItem).
    pub fn build(self) -> crate::types::RecommendationItem {
        crate::types::RecommendationItem {
            resource_id: self.resource_id,
            target_account_id: self.target_account_id,
            target_region: self.target_region,
            already_implemented: self.already_implemented,
        }
    }
}
