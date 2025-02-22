// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>DeleteLocation</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteLocationInput {
    /// <p>The Amazon Resource Name (ARN) of the location to delete.</p>
    #[doc(hidden)]
    pub location_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteLocationInput {
    /// <p>The Amazon Resource Name (ARN) of the location to delete.</p>
    pub fn location_arn(&self) -> ::std::option::Option<&str> {
        self.location_arn.as_deref()
    }
}
impl DeleteLocationInput {
    /// Creates a new builder-style object to manufacture [`DeleteLocationInput`](crate::operation::delete_location::DeleteLocationInput).
    pub fn builder() -> crate::operation::delete_location::builders::DeleteLocationInputBuilder {
        crate::operation::delete_location::builders::DeleteLocationInputBuilder::default()
    }
}

/// A builder for [`DeleteLocationInput`](crate::operation::delete_location::DeleteLocationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteLocationInputBuilder {
    pub(crate) location_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteLocationInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the location to delete.</p>
    pub fn location_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.location_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the location to delete.</p>
    pub fn set_location_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.location_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteLocationInput`](crate::operation::delete_location::DeleteLocationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_location::DeleteLocationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::delete_location::DeleteLocationInput {
            location_arn: self.location_arn,
        })
    }
}
