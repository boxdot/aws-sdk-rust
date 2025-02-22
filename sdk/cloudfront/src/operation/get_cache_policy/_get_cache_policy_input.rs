// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCachePolicyInput {
    /// <p>The unique identifier for the cache policy. If the cache policy is attached to a distribution's cache behavior, you can get the policy's identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetCachePolicyInput {
    /// <p>The unique identifier for the cache policy. If the cache policy is attached to a distribution's cache behavior, you can get the policy's identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetCachePolicyInput {
    /// Creates a new builder-style object to manufacture [`GetCachePolicyInput`](crate::operation::get_cache_policy::GetCachePolicyInput).
    pub fn builder() -> crate::operation::get_cache_policy::builders::GetCachePolicyInputBuilder {
        crate::operation::get_cache_policy::builders::GetCachePolicyInputBuilder::default()
    }
}

/// A builder for [`GetCachePolicyInput`](crate::operation::get_cache_policy::GetCachePolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCachePolicyInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetCachePolicyInputBuilder {
    /// <p>The unique identifier for the cache policy. If the cache policy is attached to a distribution's cache behavior, you can get the policy's identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the cache policy. If the cache policy is attached to a distribution's cache behavior, you can get the policy's identifier using <code>ListDistributions</code> or <code>GetDistribution</code>. If the cache policy is not attached to a cache behavior, you can get the identifier using <code>ListCachePolicies</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetCachePolicyInput`](crate::operation::get_cache_policy::GetCachePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_cache_policy::GetCachePolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_cache_policy::GetCachePolicyInput {
            id: self.id,
        })
    }
}
