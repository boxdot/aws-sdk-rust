// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeWorkspaceDirectoriesOutput {
    /// <p>Information about the directories.</p>
    #[doc(hidden)]
    pub directories: ::std::option::Option<::std::vec::Vec<crate::types::WorkspaceDirectory>>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeWorkspaceDirectoriesOutput {
    /// <p>Information about the directories.</p>
    pub fn directories(&self) -> ::std::option::Option<&[crate::types::WorkspaceDirectory]> {
        self.directories.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeWorkspaceDirectoriesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeWorkspaceDirectoriesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeWorkspaceDirectoriesOutput`](crate::operation::describe_workspace_directories::DescribeWorkspaceDirectoriesOutput).
    pub fn builder() -> crate::operation::describe_workspace_directories::builders::DescribeWorkspaceDirectoriesOutputBuilder{
        crate::operation::describe_workspace_directories::builders::DescribeWorkspaceDirectoriesOutputBuilder::default()
    }
}

/// A builder for [`DescribeWorkspaceDirectoriesOutput`](crate::operation::describe_workspace_directories::DescribeWorkspaceDirectoriesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeWorkspaceDirectoriesOutputBuilder {
    pub(crate) directories:
        ::std::option::Option<::std::vec::Vec<crate::types::WorkspaceDirectory>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeWorkspaceDirectoriesOutputBuilder {
    /// Appends an item to `directories`.
    ///
    /// To override the contents of this collection use [`set_directories`](Self::set_directories).
    ///
    /// <p>Information about the directories.</p>
    pub fn directories(mut self, input: crate::types::WorkspaceDirectory) -> Self {
        let mut v = self.directories.unwrap_or_default();
        v.push(input);
        self.directories = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the directories.</p>
    pub fn set_directories(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::WorkspaceDirectory>>,
    ) -> Self {
        self.directories = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
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
    /// Consumes the builder and constructs a [`DescribeWorkspaceDirectoriesOutput`](crate::operation::describe_workspace_directories::DescribeWorkspaceDirectoriesOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_workspace_directories::DescribeWorkspaceDirectoriesOutput {
        crate::operation::describe_workspace_directories::DescribeWorkspaceDirectoriesOutput {
            directories: self.directories,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
