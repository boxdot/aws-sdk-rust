// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the request to describe the user import job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeUserImportJobInput {
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    #[doc(hidden)]
    pub user_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The job ID for the user import job.</p>
    #[doc(hidden)]
    pub job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeUserImportJobInput {
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
    /// <p>The job ID for the user import job.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl DescribeUserImportJobInput {
    /// Creates a new builder-style object to manufacture [`DescribeUserImportJobInput`](crate::operation::describe_user_import_job::DescribeUserImportJobInput).
    pub fn builder(
    ) -> crate::operation::describe_user_import_job::builders::DescribeUserImportJobInputBuilder
    {
        crate::operation::describe_user_import_job::builders::DescribeUserImportJobInputBuilder::default()
    }
}

/// A builder for [`DescribeUserImportJobInput`](crate::operation::describe_user_import_job::DescribeUserImportJobInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeUserImportJobInputBuilder {
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
}
impl DescribeUserImportJobInputBuilder {
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user pool ID for the user pool that the users are being imported into.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// <p>The job ID for the user import job.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The job ID for the user import job.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeUserImportJobInput`](crate::operation::describe_user_import_job::DescribeUserImportJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_user_import_job::DescribeUserImportJobInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_user_import_job::DescribeUserImportJobInput {
                user_pool_id: self.user_pool_id,
                job_id: self.job_id,
            },
        )
    }
}
