// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for Unsubscribe action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UnsubscribeInput {
    /// <p>The ARN of the subscription to be deleted.</p>
    #[doc(hidden)]
    pub subscription_arn: ::std::option::Option<::std::string::String>,
}
impl UnsubscribeInput {
    /// <p>The ARN of the subscription to be deleted.</p>
    pub fn subscription_arn(&self) -> ::std::option::Option<&str> {
        self.subscription_arn.as_deref()
    }
}
impl UnsubscribeInput {
    /// Creates a new builder-style object to manufacture [`UnsubscribeInput`](crate::operation::unsubscribe::UnsubscribeInput).
    pub fn builder() -> crate::operation::unsubscribe::builders::UnsubscribeInputBuilder {
        crate::operation::unsubscribe::builders::UnsubscribeInputBuilder::default()
    }
}

/// A builder for [`UnsubscribeInput`](crate::operation::unsubscribe::UnsubscribeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UnsubscribeInputBuilder {
    pub(crate) subscription_arn: ::std::option::Option<::std::string::String>,
}
impl UnsubscribeInputBuilder {
    /// <p>The ARN of the subscription to be deleted.</p>
    pub fn subscription_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.subscription_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the subscription to be deleted.</p>
    pub fn set_subscription_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.subscription_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`UnsubscribeInput`](crate::operation::unsubscribe::UnsubscribeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::unsubscribe::UnsubscribeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::unsubscribe::UnsubscribeInput {
            subscription_arn: self.subscription_arn,
        })
    }
}
