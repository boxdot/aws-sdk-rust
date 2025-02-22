// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetWorkflowRunsOutput {
    /// <p>A list of workflow run metadata objects.</p>
    #[doc(hidden)]
    pub runs: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowRun>>,
    /// <p>A continuation token, if not all requested workflow runs have been returned.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetWorkflowRunsOutput {
    /// <p>A list of workflow run metadata objects.</p>
    pub fn runs(&self) -> ::std::option::Option<&[crate::types::WorkflowRun]> {
        self.runs.as_deref()
    }
    /// <p>A continuation token, if not all requested workflow runs have been returned.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetWorkflowRunsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetWorkflowRunsOutput {
    /// Creates a new builder-style object to manufacture [`GetWorkflowRunsOutput`](crate::operation::get_workflow_runs::GetWorkflowRunsOutput).
    pub fn builder() -> crate::operation::get_workflow_runs::builders::GetWorkflowRunsOutputBuilder
    {
        crate::operation::get_workflow_runs::builders::GetWorkflowRunsOutputBuilder::default()
    }
}

/// A builder for [`GetWorkflowRunsOutput`](crate::operation::get_workflow_runs::GetWorkflowRunsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetWorkflowRunsOutputBuilder {
    pub(crate) runs: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowRun>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetWorkflowRunsOutputBuilder {
    /// Appends an item to `runs`.
    ///
    /// To override the contents of this collection use [`set_runs`](Self::set_runs).
    ///
    /// <p>A list of workflow run metadata objects.</p>
    pub fn runs(mut self, input: crate::types::WorkflowRun) -> Self {
        let mut v = self.runs.unwrap_or_default();
        v.push(input);
        self.runs = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of workflow run metadata objects.</p>
    pub fn set_runs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::WorkflowRun>>,
    ) -> Self {
        self.runs = input;
        self
    }
    /// <p>A continuation token, if not all requested workflow runs have been returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A continuation token, if not all requested workflow runs have been returned.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetWorkflowRunsOutput`](crate::operation::get_workflow_runs::GetWorkflowRunsOutput).
    pub fn build(self) -> crate::operation::get_workflow_runs::GetWorkflowRunsOutput {
        crate::operation::get_workflow_runs::GetWorkflowRunsOutput {
            runs: self.runs,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
