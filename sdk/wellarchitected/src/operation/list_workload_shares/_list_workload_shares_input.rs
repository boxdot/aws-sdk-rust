// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Input for List Workload Share</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListWorkloadSharesInput {
    /// <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    #[doc(hidden)]
    pub workload_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services account ID, IAM role, organization ID, or organizational unit (OU) ID with which the workload is shared.</p>
    #[doc(hidden)]
    pub shared_with_prefix: ::std::option::Option<::std::string::String>,
    /// <p>The token to use to retrieve the next set of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return for this request.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The status of a workload share.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ShareStatus>,
}
impl ListWorkloadSharesInput {
    /// <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    pub fn workload_id(&self) -> ::std::option::Option<&str> {
        self.workload_id.as_deref()
    }
    /// <p>The Amazon Web Services account ID, IAM role, organization ID, or organizational unit (OU) ID with which the workload is shared.</p>
    pub fn shared_with_prefix(&self) -> ::std::option::Option<&str> {
        self.shared_with_prefix.as_deref()
    }
    /// <p>The token to use to retrieve the next set of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return for this request.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The status of a workload share.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ShareStatus> {
        self.status.as_ref()
    }
}
impl ListWorkloadSharesInput {
    /// Creates a new builder-style object to manufacture [`ListWorkloadSharesInput`](crate::operation::list_workload_shares::ListWorkloadSharesInput).
    pub fn builder(
    ) -> crate::operation::list_workload_shares::builders::ListWorkloadSharesInputBuilder {
        crate::operation::list_workload_shares::builders::ListWorkloadSharesInputBuilder::default()
    }
}

/// A builder for [`ListWorkloadSharesInput`](crate::operation::list_workload_shares::ListWorkloadSharesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListWorkloadSharesInputBuilder {
    pub(crate) workload_id: ::std::option::Option<::std::string::String>,
    pub(crate) shared_with_prefix: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) status: ::std::option::Option<crate::types::ShareStatus>,
}
impl ListWorkloadSharesInputBuilder {
    /// <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    pub fn workload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workload_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID assigned to the workload. This ID is unique within an Amazon Web Services Region.</p>
    pub fn set_workload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workload_id = input;
        self
    }
    /// <p>The Amazon Web Services account ID, IAM role, organization ID, or organizational unit (OU) ID with which the workload is shared.</p>
    pub fn shared_with_prefix(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.shared_with_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID, IAM role, organization ID, or organizational unit (OU) ID with which the workload is shared.</p>
    pub fn set_shared_with_prefix(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.shared_with_prefix = input;
        self
    }
    /// <p>The token to use to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return for this request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return for this request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The status of a workload share.</p>
    pub fn status(mut self, input: crate::types::ShareStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of a workload share.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ShareStatus>) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`ListWorkloadSharesInput`](crate::operation::list_workload_shares::ListWorkloadSharesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_workload_shares::ListWorkloadSharesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_workload_shares::ListWorkloadSharesInput {
                workload_id: self.workload_id,
                shared_with_prefix: self.shared_with_prefix,
                next_token: self.next_token,
                max_results: self.max_results,
                status: self.status,
            },
        )
    }
}
