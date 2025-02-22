// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGroupInput {
    /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains numbers and lower case letters. This value is generated at the time that a new identity store is created.</p>
    #[doc(hidden)]
    pub identity_store_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for a group in the identity store.</p>
    #[doc(hidden)]
    pub group_id: ::std::option::Option<::std::string::String>,
}
impl DescribeGroupInput {
    /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains numbers and lower case letters. This value is generated at the time that a new identity store is created.</p>
    pub fn identity_store_id(&self) -> ::std::option::Option<&str> {
        self.identity_store_id.as_deref()
    }
    /// <p>The identifier for a group in the identity store.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
}
impl DescribeGroupInput {
    /// Creates a new builder-style object to manufacture [`DescribeGroupInput`](crate::operation::describe_group::DescribeGroupInput).
    pub fn builder() -> crate::operation::describe_group::builders::DescribeGroupInputBuilder {
        crate::operation::describe_group::builders::DescribeGroupInputBuilder::default()
    }
}

/// A builder for [`DescribeGroupInput`](crate::operation::describe_group::DescribeGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeGroupInputBuilder {
    pub(crate) identity_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
}
impl DescribeGroupInputBuilder {
    /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains numbers and lower case letters. This value is generated at the time that a new identity store is created.</p>
    pub fn identity_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.identity_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The globally unique identifier for the identity store, such as <code>d-1234567890</code>. In this example, <code>d-</code> is a fixed prefix, and <code>1234567890</code> is a randomly generated string that contains numbers and lower case letters. This value is generated at the time that a new identity store is created.</p>
    pub fn set_identity_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.identity_store_id = input;
        self
    }
    /// <p>The identifier for a group in the identity store.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for a group in the identity store.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeGroupInput`](crate::operation::describe_group::DescribeGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_group::DescribeGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_group::DescribeGroupInput {
            identity_store_id: self.identity_store_id,
            group_id: self.group_id,
        })
    }
}
