// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteLifecyclePolicyInput {
    /// <p>The Amazon Web Services account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    #[doc(hidden)]
    pub registry_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the repository.</p>
    #[doc(hidden)]
    pub repository_name: ::std::option::Option<::std::string::String>,
}
impl DeleteLifecyclePolicyInput {
    /// <p>The Amazon Web Services account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    pub fn registry_id(&self) -> ::std::option::Option<&str> {
        self.registry_id.as_deref()
    }
    /// <p>The name of the repository.</p>
    pub fn repository_name(&self) -> ::std::option::Option<&str> {
        self.repository_name.as_deref()
    }
}
impl DeleteLifecyclePolicyInput {
    /// Creates a new builder-style object to manufacture [`DeleteLifecyclePolicyInput`](crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyInput).
    pub fn builder(
    ) -> crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyInputBuilder
    {
        crate::operation::delete_lifecycle_policy::builders::DeleteLifecyclePolicyInputBuilder::default()
    }
}

/// A builder for [`DeleteLifecyclePolicyInput`](crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteLifecyclePolicyInputBuilder {
    pub(crate) registry_id: ::std::option::Option<::std::string::String>,
    pub(crate) repository_name: ::std::option::Option<::std::string::String>,
}
impl DeleteLifecyclePolicyInputBuilder {
    /// <p>The Amazon Web Services account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    pub fn registry_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.registry_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID associated with the registry that contains the repository. If you do not specify a registry, the default registry is assumed.</p>
    pub fn set_registry_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.registry_id = input;
        self
    }
    /// <p>The name of the repository.</p>
    pub fn repository_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.repository_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the repository.</p>
    pub fn set_repository_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.repository_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteLifecyclePolicyInput`](crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_lifecycle_policy::DeleteLifecyclePolicyInput {
                registry_id: self.registry_id,
                repository_name: self.repository_name,
            },
        )
    }
}
