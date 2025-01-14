// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribePublisherInput {
    /// <p>The ID of the extension publisher.</p>
    /// <p>If you don't supply a <code>PublisherId</code>, and you have registered as an extension publisher, <code>DescribePublisher</code> returns information about your own publisher account.</p>
    #[doc(hidden)]
    pub publisher_id: ::std::option::Option<::std::string::String>,
}
impl DescribePublisherInput {
    /// <p>The ID of the extension publisher.</p>
    /// <p>If you don't supply a <code>PublisherId</code>, and you have registered as an extension publisher, <code>DescribePublisher</code> returns information about your own publisher account.</p>
    pub fn publisher_id(&self) -> ::std::option::Option<&str> {
        self.publisher_id.as_deref()
    }
}
impl DescribePublisherInput {
    /// Creates a new builder-style object to manufacture [`DescribePublisherInput`](crate::operation::describe_publisher::DescribePublisherInput).
    pub fn builder() -> crate::operation::describe_publisher::builders::DescribePublisherInputBuilder
    {
        crate::operation::describe_publisher::builders::DescribePublisherInputBuilder::default()
    }
}

/// A builder for [`DescribePublisherInput`](crate::operation::describe_publisher::DescribePublisherInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribePublisherInputBuilder {
    pub(crate) publisher_id: ::std::option::Option<::std::string::String>,
}
impl DescribePublisherInputBuilder {
    /// <p>The ID of the extension publisher.</p>
    /// <p>If you don't supply a <code>PublisherId</code>, and you have registered as an extension publisher, <code>DescribePublisher</code> returns information about your own publisher account.</p>
    pub fn publisher_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.publisher_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the extension publisher.</p>
    /// <p>If you don't supply a <code>PublisherId</code>, and you have registered as an extension publisher, <code>DescribePublisher</code> returns information about your own publisher account.</p>
    pub fn set_publisher_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.publisher_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribePublisherInput`](crate::operation::describe_publisher::DescribePublisherInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_publisher::DescribePublisherInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_publisher::DescribePublisherInput {
                publisher_id: self.publisher_id,
            },
        )
    }
}
