// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeClusterInput {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    #[doc(hidden)]
    pub cluster_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeClusterInput {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn cluster_arn(&self) -> ::std::option::Option<&str> {
        self.cluster_arn.as_deref()
    }
}
impl DescribeClusterInput {
    /// Creates a new builder-style object to manufacture [`DescribeClusterInput`](crate::operation::describe_cluster::DescribeClusterInput).
    pub fn builder() -> crate::operation::describe_cluster::builders::DescribeClusterInputBuilder {
        crate::operation::describe_cluster::builders::DescribeClusterInputBuilder::default()
    }
}

/// A builder for [`DescribeClusterInput`](crate::operation::describe_cluster::DescribeClusterInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeClusterInputBuilder {
    pub(crate) cluster_arn: ::std::option::Option<::std::string::String>,
}
impl DescribeClusterInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn cluster_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the cluster.</p>
    pub fn set_cluster_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeClusterInput`](crate::operation::describe_cluster::DescribeClusterInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_cluster::DescribeClusterInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_cluster::DescribeClusterInput {
            cluster_arn: self.cluster_arn,
        })
    }
}
