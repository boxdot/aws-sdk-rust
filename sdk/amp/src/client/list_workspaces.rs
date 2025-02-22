// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListWorkspaces`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::set_next_token): Pagination token to request the next page in a paginated list. This token is obtained from the output of the previous ListWorkspaces request.
    ///   - [`alias(impl ::std::convert::Into<String>)`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::alias) / [`set_alias(Option<String>)`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::set_alias): Optional filter for workspace alias. Only the workspaces with aliases that begin with this value will be returned.
    ///   - [`max_results(i32)`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::set_max_results): Maximum results to return in response (default=100, maximum=1000).
    /// - On success, responds with [`ListWorkspacesOutput`](crate::operation::list_workspaces::ListWorkspacesOutput) with field(s):
    ///   - [`workspaces(Option<Vec<WorkspaceSummary>>)`](crate::operation::list_workspaces::ListWorkspacesOutput::workspaces): The list of existing workspaces, including those undergoing creation or deletion.
    ///   - [`next_token(Option<String>)`](crate::operation::list_workspaces::ListWorkspacesOutput::next_token): Pagination token to use when requesting the next page in this list.
    /// - On failure, responds with [`SdkError<ListWorkspacesError>`](crate::operation::list_workspaces::ListWorkspacesError)
    pub fn list_workspaces(
        &self,
    ) -> crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder {
        crate::operation::list_workspaces::builders::ListWorkspacesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
