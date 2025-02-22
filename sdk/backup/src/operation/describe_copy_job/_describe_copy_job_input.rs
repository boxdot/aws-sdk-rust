// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeCopyJobInput {
    /// <p>Uniquely identifies a copy job.</p>
    #[doc(hidden)]
    pub copy_job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeCopyJobInput {
    /// <p>Uniquely identifies a copy job.</p>
    pub fn copy_job_id(&self) -> ::std::option::Option<&str> {
        self.copy_job_id.as_deref()
    }
}
impl DescribeCopyJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeCopyJobInput`](crate::operation::describe_copy_job::DescribeCopyJobInput).
    pub fn builder() -> crate::operation::describe_copy_job::builders::DescribeCopyJobInputBuilder {
        crate::operation::describe_copy_job::builders::DescribeCopyJobInputBuilder::default()
    }
}

/// A builder for [`DescribeCopyJobInput`](crate::operation::describe_copy_job::DescribeCopyJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeCopyJobInputBuilder {
    pub(crate) copy_job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeCopyJobInputBuilder {
    /// <p>Uniquely identifies a copy job.</p>
    pub fn copy_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.copy_job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Uniquely identifies a copy job.</p>
    pub fn set_copy_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.copy_job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeCopyJobInput`](crate::operation::describe_copy_job::DescribeCopyJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_copy_job::DescribeCopyJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_copy_job::DescribeCopyJobInput {
            copy_job_id: self.copy_job_id,
        })
    }
}
