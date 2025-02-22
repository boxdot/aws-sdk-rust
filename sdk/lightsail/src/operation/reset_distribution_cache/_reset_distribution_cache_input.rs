// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResetDistributionCacheInput {
    /// <p>The name of the distribution for which to reset cache.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    #[doc(hidden)]
    pub distribution_name: ::std::option::Option<::std::string::String>,
}
impl ResetDistributionCacheInput {
    /// <p>The name of the distribution for which to reset cache.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn distribution_name(&self) -> ::std::option::Option<&str> {
        self.distribution_name.as_deref()
    }
}
impl ResetDistributionCacheInput {
    /// Creates a new builder-style object to manufacture [`ResetDistributionCacheInput`](crate::operation::reset_distribution_cache::ResetDistributionCacheInput).
    pub fn builder(
    ) -> crate::operation::reset_distribution_cache::builders::ResetDistributionCacheInputBuilder
    {
        crate::operation::reset_distribution_cache::builders::ResetDistributionCacheInputBuilder::default()
    }
}

/// A builder for [`ResetDistributionCacheInput`](crate::operation::reset_distribution_cache::ResetDistributionCacheInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResetDistributionCacheInputBuilder {
    pub(crate) distribution_name: ::std::option::Option<::std::string::String>,
}
impl ResetDistributionCacheInputBuilder {
    /// <p>The name of the distribution for which to reset cache.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn distribution_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.distribution_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the distribution for which to reset cache.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn set_distribution_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.distribution_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ResetDistributionCacheInput`](crate::operation::reset_distribution_cache::ResetDistributionCacheInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reset_distribution_cache::ResetDistributionCacheInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::reset_distribution_cache::ResetDistributionCacheInput {
                distribution_name: self.distribution_name,
            },
        )
    }
}
