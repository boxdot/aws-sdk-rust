// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about the Amazon ElastiCache instances that Amazon Web Services recommends that you purchase.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ElastiCacheInstanceDetails {
    /// <p>The instance family of the recommended reservation.</p>
    #[doc(hidden)]
    pub family: ::std::option::Option<::std::string::String>,
    /// <p>The type of node that Amazon Web Services recommends.</p>
    #[doc(hidden)]
    pub node_type: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region of the recommended reservation.</p>
    #[doc(hidden)]
    pub region: ::std::option::Option<::std::string::String>,
    /// <p>The description of the recommended reservation.</p>
    #[doc(hidden)]
    pub product_description: ::std::option::Option<::std::string::String>,
    /// <p>Determines whether the recommendation is for a current generation instance.</p>
    #[doc(hidden)]
    pub current_generation: bool,
    /// <p>Determines whether the recommended reservation is size flexible.</p>
    #[doc(hidden)]
    pub size_flex_eligible: bool,
}
impl ElastiCacheInstanceDetails {
    /// <p>The instance family of the recommended reservation.</p>
    pub fn family(&self) -> ::std::option::Option<&str> {
        self.family.as_deref()
    }
    /// <p>The type of node that Amazon Web Services recommends.</p>
    pub fn node_type(&self) -> ::std::option::Option<&str> {
        self.node_type.as_deref()
    }
    /// <p>The Amazon Web Services Region of the recommended reservation.</p>
    pub fn region(&self) -> ::std::option::Option<&str> {
        self.region.as_deref()
    }
    /// <p>The description of the recommended reservation.</p>
    pub fn product_description(&self) -> ::std::option::Option<&str> {
        self.product_description.as_deref()
    }
    /// <p>Determines whether the recommendation is for a current generation instance.</p>
    pub fn current_generation(&self) -> bool {
        self.current_generation
    }
    /// <p>Determines whether the recommended reservation is size flexible.</p>
    pub fn size_flex_eligible(&self) -> bool {
        self.size_flex_eligible
    }
}
impl ElastiCacheInstanceDetails {
    /// Creates a new builder-style object to manufacture [`ElastiCacheInstanceDetails`](crate::types::ElastiCacheInstanceDetails).
    pub fn builder() -> crate::types::builders::ElastiCacheInstanceDetailsBuilder {
        crate::types::builders::ElastiCacheInstanceDetailsBuilder::default()
    }
}

/// A builder for [`ElastiCacheInstanceDetails`](crate::types::ElastiCacheInstanceDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ElastiCacheInstanceDetailsBuilder {
    pub(crate) family: ::std::option::Option<::std::string::String>,
    pub(crate) node_type: ::std::option::Option<::std::string::String>,
    pub(crate) region: ::std::option::Option<::std::string::String>,
    pub(crate) product_description: ::std::option::Option<::std::string::String>,
    pub(crate) current_generation: ::std::option::Option<bool>,
    pub(crate) size_flex_eligible: ::std::option::Option<bool>,
}
impl ElastiCacheInstanceDetailsBuilder {
    /// <p>The instance family of the recommended reservation.</p>
    pub fn family(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.family = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instance family of the recommended reservation.</p>
    pub fn set_family(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.family = input;
        self
    }
    /// <p>The type of node that Amazon Web Services recommends.</p>
    pub fn node_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.node_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of node that Amazon Web Services recommends.</p>
    pub fn set_node_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.node_type = input;
        self
    }
    /// <p>The Amazon Web Services Region of the recommended reservation.</p>
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region of the recommended reservation.</p>
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// <p>The description of the recommended reservation.</p>
    pub fn product_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.product_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the recommended reservation.</p>
    pub fn set_product_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.product_description = input;
        self
    }
    /// <p>Determines whether the recommendation is for a current generation instance.</p>
    pub fn current_generation(mut self, input: bool) -> Self {
        self.current_generation = ::std::option::Option::Some(input);
        self
    }
    /// <p>Determines whether the recommendation is for a current generation instance.</p>
    pub fn set_current_generation(mut self, input: ::std::option::Option<bool>) -> Self {
        self.current_generation = input;
        self
    }
    /// <p>Determines whether the recommended reservation is size flexible.</p>
    pub fn size_flex_eligible(mut self, input: bool) -> Self {
        self.size_flex_eligible = ::std::option::Option::Some(input);
        self
    }
    /// <p>Determines whether the recommended reservation is size flexible.</p>
    pub fn set_size_flex_eligible(mut self, input: ::std::option::Option<bool>) -> Self {
        self.size_flex_eligible = input;
        self
    }
    /// Consumes the builder and constructs a [`ElastiCacheInstanceDetails`](crate::types::ElastiCacheInstanceDetails).
    pub fn build(self) -> crate::types::ElastiCacheInstanceDetails {
        crate::types::ElastiCacheInstanceDetails {
            family: self.family,
            node_type: self.node_type,
            region: self.region,
            product_description: self.product_description,
            current_generation: self.current_generation.unwrap_or_default(),
            size_flex_eligible: self.size_flex_eligible.unwrap_or_default(),
        }
    }
}
