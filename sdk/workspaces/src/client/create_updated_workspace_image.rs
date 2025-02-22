// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateUpdatedWorkspaceImage`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::set_name): <p>The name of the new updated WorkSpace image.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::set_description): <p>A description of whether updates for the WorkSpace image are available.</p>
    ///   - [`source_image_id(impl ::std::convert::Into<String>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::source_image_id) / [`set_source_image_id(Option<String>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::set_source_image_id): <p>The identifier of the source WorkSpace image.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::set_tags): <p>The tags that you want to add to the new updated WorkSpace image.</p> <note>   <p>To add tags at the same time when you're creating the updated image, you must create an IAM policy that grants your IAM user permissions to use <code>workspaces:CreateTags</code>. </p>  </note>
    /// - On success, responds with [`CreateUpdatedWorkspaceImageOutput`](crate::operation::create_updated_workspace_image::CreateUpdatedWorkspaceImageOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::operation::create_updated_workspace_image::CreateUpdatedWorkspaceImageOutput::image_id): <p>The identifier of the new updated WorkSpace image.</p>
    /// - On failure, responds with [`SdkError<CreateUpdatedWorkspaceImageError>`](crate::operation::create_updated_workspace_image::CreateUpdatedWorkspaceImageError)
    pub fn create_updated_workspace_image(&self) -> crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder{
        crate::operation::create_updated_workspace_image::builders::CreateUpdatedWorkspaceImageFluentBuilder::new(self.handle.clone())
    }
}
