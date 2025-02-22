// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTopicPermissionsInput {
    /// <p>The ID of the Amazon Web Services account that contains the topic that you want described.</p>
    #[doc(hidden)]
    pub aws_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the topic that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    #[doc(hidden)]
    pub topic_id: ::std::option::Option<::std::string::String>,
}
impl DescribeTopicPermissionsInput {
    /// <p>The ID of the Amazon Web Services account that contains the topic that you want described.</p>
    pub fn aws_account_id(&self) -> ::std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
    /// <p>The ID of the topic that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn topic_id(&self) -> ::std::option::Option<&str> {
        self.topic_id.as_deref()
    }
}
impl DescribeTopicPermissionsInput {
    /// Creates a new builder-style object to manufacture [`DescribeTopicPermissionsInput`](crate::operation::describe_topic_permissions::DescribeTopicPermissionsInput).
    pub fn builder(
    ) -> crate::operation::describe_topic_permissions::builders::DescribeTopicPermissionsInputBuilder
    {
        crate::operation::describe_topic_permissions::builders::DescribeTopicPermissionsInputBuilder::default()
    }
}

/// A builder for [`DescribeTopicPermissionsInput`](crate::operation::describe_topic_permissions::DescribeTopicPermissionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeTopicPermissionsInputBuilder {
    pub(crate) aws_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) topic_id: ::std::option::Option<::std::string::String>,
}
impl DescribeTopicPermissionsInputBuilder {
    /// <p>The ID of the Amazon Web Services account that contains the topic that you want described.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.aws_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the topic that you want described.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.aws_account_id = input;
        self
    }
    /// <p>The ID of the topic that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn topic_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.topic_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the topic that you want to describe. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn set_topic_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.topic_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeTopicPermissionsInput`](crate::operation::describe_topic_permissions::DescribeTopicPermissionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_topic_permissions::DescribeTopicPermissionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_topic_permissions::DescribeTopicPermissionsInput {
                aws_account_id: self.aws_account_id,
                topic_id: self.topic_id,
            },
        )
    }
}
