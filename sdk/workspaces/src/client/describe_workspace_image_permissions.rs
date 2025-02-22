// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeWorkspaceImagePermissions`](crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`image_id(impl ::std::convert::Into<String>)`](crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder::image_id) / [`set_image_id(Option<String>)`](crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder::set_image_id): <p>The identifier of the image.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder::set_next_token): <p>If you received a <code>NextToken</code> from a previous call that was paginated, provide this token to receive the next set of results.</p>
    ///   - [`max_results(i32)`](crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder::set_max_results): <p>The maximum number of items to return.</p>
    /// - On success, responds with [`DescribeWorkspaceImagePermissionsOutput`](crate::operation::describe_workspace_image_permissions::DescribeWorkspaceImagePermissionsOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::operation::describe_workspace_image_permissions::DescribeWorkspaceImagePermissionsOutput::image_id): <p>The identifier of the image.</p>
    ///   - [`image_permissions(Option<Vec<ImagePermission>>)`](crate::operation::describe_workspace_image_permissions::DescribeWorkspaceImagePermissionsOutput::image_permissions): <p>The identifiers of the Amazon Web Services accounts that the image has been shared with.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_workspace_image_permissions::DescribeWorkspaceImagePermissionsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return. </p>
    /// - On failure, responds with [`SdkError<DescribeWorkspaceImagePermissionsError>`](crate::operation::describe_workspace_image_permissions::DescribeWorkspaceImagePermissionsError)
    pub fn describe_workspace_image_permissions(&self) -> crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder{
        crate::operation::describe_workspace_image_permissions::builders::DescribeWorkspaceImagePermissionsFluentBuilder::new(self.handle.clone())
    }
}
